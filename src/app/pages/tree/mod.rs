use egui::{
    Align2, Color32, Image, Layout, OpenUrl, Sense, StrokeKind, UiBuilder, Widget as _,
    epaint::Hsva, pos2, vec2,
};
use serde::{Deserialize, Serialize};
use wiki_data::{
    ITEMS, Item,
    item::{Rarity, RarityColor},
};

use crate::{TerratreeApp, app::Dragging};

mod node_by_name;

// #[derive(Serialize, Deserialize)]
// pub struct Milestone {
//     pub image: String,
//     pub title: String,
//     #[serde(with = "node_by_name")]
//     pub roots: Vec<Node>,
// }

#[derive(Serialize, Deserialize)]
pub struct Node {
    #[serde(with = "node_by_name")]
    pub item: &'static Item,
    pub next: Option<Box<Node>>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn from_item(item: &'static Item) -> Self {
        Self {
            item,
            next: None,
            children: Vec::new(),
        }
    }

    pub fn from_name(name: &'static str) -> Option<Self> {
        ITEMS
            .iter()
            .find(|item| item.name == name)
            .map(Self::from_item)
    }

    pub fn ui(&mut self, dragging: &mut Dragging, ui: &mut egui::Ui) -> egui::Response {
        let img = self.item.image_location.as_ref().unwrap();

        let image_res = Image::new(format!("wiki://{}", img.name))
            .fit_to_exact_size(vec2(48., 48.))
            .ui(ui);

        if let Dragging::Item(item) = dragging
            && ui.ctx().dragged_id().is_none()
            && image_res.contains_pointer()
        {
            self.children.push(Node::from_item(item));
        }

        let is_dragging_item = matches!(dragging, Dragging::Item(_));

        let main_line_x = image_res.rect.center().x;
        let mut last_line_pos = None;

        let child_rect = {
            let mut child_rect = ui.available_rect_before_wrap();
            child_rect.set_left(child_rect.left() + 48.);

            let mut ui = ui.new_child(
                UiBuilder::new()
                    .layout(Layout::top_down(egui::Align::LEFT))
                    .max_rect(child_rect),
            );
            let ui = &mut ui;

            let mut move_to_top = None;
            let mut index = 0;
            self.children.retain_mut(|child| {
                let res = child.ui(dragging, ui);

                let pos = pos2(main_line_x, res.rect.center().y);
                last_line_pos = Some(pos);
                ui.painter().line_segment(
                    [res.rect.left_center(), pos],
                    ui.visuals().widgets.inactive.fg_stroke,
                );

                res.context_menu(|ui| {
                    if ui.button("To top").clicked() {
                        move_to_top = Some(index);
                    }
                });

                index += 1;
                !res.clicked_by(egui::PointerButton::Middle)
            });

            if let Some(i) = move_to_top {
                let child = self.children.remove(i);
                self.children.insert(0, child);
            }

            if is_dragging_item {
                let (res, item) = item_drag_target(ui, dragging);

                ui.painter().line_segment(
                    [res.rect.left_center(), pos2(main_line_x, res.rect.center().y)],
                    ui.visuals().widgets.inactive.fg_stroke,
                );

                if let Some(item) = item {
                    self.children.push(Node::from_item(item));
                }
            }

            ui.min_rect()
        };
        ui.allocate_rect(child_rect, Sense::hover());

        if self.next.is_some() || is_dragging_item {
            ui.painter().line_segment(
                [image_res.rect.center_bottom(), pos2(main_line_x, ui.cursor().top())],
                ui.visuals().widgets.active.fg_stroke,
            );
        } else if let Some(pos) = last_line_pos {
            ui.painter().line_segment(
                [image_res.rect.center_bottom(), pos],
                ui.visuals().widgets.inactive.fg_stroke,
            );
        }

        if let Some(ref mut first_child) = self.next {
            let res = first_child.ui(dragging, ui);

            if res.clicked_by(egui::PointerButton::Middle) {
                self.next = None;
            }
        } else if is_dragging_item {
            let (_res, item) = item_drag_target(ui, dragging);

            if let Some(item) = item {
                self.next = Some(Box::new(Node::from_item(item)));
            }
        }

        image_res.interact(Sense::click())
    }
}

pub fn item_drag_target(
    ui: &mut egui::Ui,
    dragging: &mut Dragging,
) -> (egui::Response, Option<&'static Item>) {
    let (res, painter) = ui.allocate_painter(vec2(48., 48.), Sense::hover());

    if let Dragging::Item(item) = dragging {
        let rect = res.rect.shrink(4.);
        let style = if res.contains_pointer() {
            ui.visuals().widgets.active
        } else {
            ui.visuals().widgets.inactive
        };

        painter.rect(rect, style.corner_radius, style.bg_fill, style.bg_stroke, StrokeKind::Inside);

        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            "+",
            ui.style().text_styles[&egui::TextStyle::Heading].clone(),
            style.fg_stroke.color,
        );

        if ui.ctx().dragged_id().is_none() && res.contains_pointer() {
            return (res, Some(item));
        }
    }

    (res, None)
}

pub fn ui(ctx: &egui::Context, app: &mut TerratreeApp) {
    egui::SidePanel::right("items").show(ctx, |ui| {
        ui.heading("Items");

        egui::TextEdit::singleline(&mut app.ui_state.item_search)
            .hint_text("Search")
            .show(ui);

        ui.add_space(2.);

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(
                egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                |ui| {
                    for item in wiki_data::ITEMS.iter() {
                        if !item
                            .name
                            .to_lowercase()
                            .contains(&app.ui_state.item_search.to_lowercase())
                        {
                            continue;
                        }

                        let res = draw_sidebar_item(ui, item);

                        if res.drag_started() {
                            ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Grab);
                            app.dragging = Dragging::Item(item);
                        }

                        res.context_menu(|ui| {
                            if ui.button("Open Wiki").clicked() {
                                let url = format!(
                                    "https://terraria.wiki.gg/wiki/{}",
                                    item.name.replace(' ', "_")
                                );
                                ui.ctx().open_url(OpenUrl::new_tab(url));
                            }
                        });
                    }
                },
            );
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        app.tree_root.ui(&mut app.dragging, ui);

        // app.milestones.retain_mut(|milestone| {
        //     let mut delete = false;

        //     let res = ui
        //         .horizontal(|ui| {
        //             egui::Image::new(&milestone.image)
        //                 .fit_to_exact_size(vec2(32., 32.))
        //                 .ui(ui);

        //             ui.heading(&milestone.title);

        //             ui.take_available_width();
        //         })
        //         .response
        //         .interact(Sense::click());

        //     res.context_menu(|ui| {
        //         let res = ui.text_edit_singleline(&mut milestone.title);
        //         if ui.memory(|m| m.focused().is_none()) {
        //             res.request_focus();
        //         }
        //         if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        //             ui.close();
        //         }

        //         if ui.button("Delete").clicked() {
        //             delete = true;
        //         }
        //     });

        //     !delete
        // });

        // // new milestone drop target
        // {
        //     let (res, painter) =
        //         ui.allocate_painter(vec2(ui.available_width(), 48.), Sense::hover());

        //     if let Dragging::Item(item) = app.dragging {
        //         let rect = res.rect.shrink(4.);
        //         let style = if res.contains_pointer() {
        //             ui.visuals().widgets.active
        //         } else {
        //             ui.visuals().widgets.inactive
        //         };

        //         painter.rect(
        //             rect,
        //             style.corner_radius,
        //             style.bg_fill,
        //             style.bg_stroke,
        //             StrokeKind::Inside,
        //         );

        //         painter.text(
        //             rect.center(),
        //             Align2::CENTER_CENTER,
        //             "Create milestone",
        //             ui.style().text_styles[&egui::TextStyle::Heading].clone(),
        //             style.fg_stroke.color,
        //         );

        //         if ctx.dragged_id().is_none() && res.contains_pointer() {
        //             app.milestones.push(Node { item, children: Vec::new() });
        //         }
        //     }
        // }
    });
}

fn draw_sidebar_item(ui: &mut egui::Ui, item: &wiki_data::item::Item) -> egui::Response {
    let size = 32.;
    let margin = ui.spacing().item_spacing.y;
    let (res, painter) =
        ui.allocate_painter(vec2(size * 2., size + margin * 2.), Sense::click_and_drag());

    let rect = res.rect.shrink2(vec2(0., margin));

    if !rect.intersects(painter.clip_rect()) {
        return res;
    }

    let (img_rect, info_rect) = rect.split_left_right_at_x(rect.left() + rect.height());

    if let Some(image) = item.image_location.as_ref() {
        ui.place(img_rect, egui::Image::new(format!("wiki://{}", image.name)));
    }

    let info_rect = info_rect.shrink2(vec2(8., 3.));

    let color = rarity_color(ui, item.rarity);

    painter.text(
        info_rect.left_top(),
        Align2::LEFT_TOP,
        &item.name,
        ui.style().text_styles[&egui::TextStyle::Body].clone(),
        color,
    );

    let subtext = [
        item.tooltip
            .as_ref()
            .and_then(|t| t.plain().split('\n').next().map(|s| s.to_string()))
            .unwrap_or_default(),
        item.damage
            .as_ref()
            .map(|d| {
                let ty = item
                    .damage_type
                    .as_ref()
                    .map(|t| format!("{t} ").to_lowercase())
                    .unwrap_or_default();

                format!("{d} {ty}damage")
            })
            .unwrap_or_default(),
        item.types
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(", "),
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>()
    .join(" · ");

    painter.text(
        info_rect.left_bottom(),
        Align2::LEFT_BOTTOM,
        subtext,
        ui.style().text_styles[&egui::TextStyle::Small].clone(),
        ui.visuals()
            .weak_text_color
            .unwrap_or(ui.visuals().text_color()),
    );

    res
}

fn rarity_color(ui: &egui::Ui, rarity: Rarity) -> Color32 {
    match rarity.color() {
        RarityColor::Static { r, g, b } => Color32::from_rgb(r, g, b),
        RarityColor::Expert => {
            ui.ctx().request_repaint();
            Hsva::new(((ui.input(|i| i.time) / 2.) % 1.) as f32, 1., 1., 1.).into()
        }
        RarityColor::Master => {
            ui.ctx().request_repaint();
            Hsva::new((((ui.input(|i| i.time) / 2.) % 0.3) as f32 - 0.15).abs(), 1., 1., 1.).into()
        }
    }
}
