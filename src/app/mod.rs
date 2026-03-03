use std::sync::Arc;

use egui::emath::GuiRounding;
use egui::epaint::Hsva;
use egui::load::TexturePoll;
use egui::{Align2, Color32, Sense, vec2};
use serde::{Deserialize, Serialize};
use wiki_data::item::RarityColor;

use crate::tdata::{self, tree::TDataTree};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct TemplateApp {
    #[serde(skip)]
    pub tree: TDataTree,
    pub item_search: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            tree: TDataTree::from_tdata(&tdata::Data::load().unwrap()),
            item_search: String::new(),
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        egui_extras::install_image_loaders(&cc.egui_ctx);

        cc.egui_ctx
            .add_bytes_loader(Arc::new(crate::wiki_img::Loader::new()));

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     egui::menu::bar(ui, |ui| {
        //         if cfg!(not(target_arch = "wasm32")) {
        //             ui.menu_button("File", |ui| {
        //                 if ui.button("Quit").clicked() {
        //                     ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        //                 }
        //             });
        //             ui.add_space(16.0);
        //         }

        //         egui::widgets::global_theme_preference_buttons(ui);
        //     });
        // });

        egui::SidePanel::right("items").show(ctx, |ui| {
            ui.heading("Items");

            egui::TextEdit::singleline(&mut self.item_search)
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
                                .contains(&self.item_search.to_lowercase())
                            {
                                continue;
                            }

                            let size = 32.;
                            let margin = ui.spacing().item_spacing.y;
                            let (res, painter) = ui.allocate_painter(
                                vec2(size * 2., size + margin * 2.),
                                Sense::click(),
                            );

                            let rect = res.rect.shrink2(vec2(0., margin));

                            if !rect.intersects(painter.clip_rect()) {
                                continue;
                            }

                            let (img_rect, info_rect) =
                                rect.split_left_right_at_x(rect.left() + rect.height());

                            if let Some(image) = item.image_location.as_ref() {
                                ui.place(
                                    img_rect,
                                    egui::Image::new(format!("wiki://{}", image.name)),
                                );
                            }

                            let info_rect = info_rect.shrink2(vec2(8., 3.));

                            let color = match item.rarity.color() {
                                RarityColor::Static { r, g, b } => Color32::from_rgb(r, g, b),
                                RarityColor::Expert => {
                                    ui.ctx().request_repaint();
                                    Hsva::new(((ui.input(|i| i.time) / 2.) % 1.) as f32, 1., 1., 1.)
                                        .into()
                                }
                                RarityColor::Master => {
                                    ui.ctx().request_repaint();
                                    Hsva::new(
                                        (((ui.input(|i| i.time) / 2.) % 0.3) as f32 - 0.15).abs(),
                                        1.,
                                        1.,
                                        1.,
                                    )
                                    .into()
                                }
                            };

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
                                    .and_then(|t| {
                                        t.plain().split('\n').next().map(|s| s.to_string())
                                    })
                                    .unwrap_or_default(),
                                item.types
                                    .iter()
                                    .map(|t| t.to_string())
                                    .collect::<Vec<_>>()
                                    .join(", "),
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
                        }
                    },
                );
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(ui.ctx().pixels_per_point().to_string());
            for milestone in &self.tree.milestones {
                ui.heading(&milestone.name);

                for item in &milestone.items {
                    if let Some(image) = item.wiki_data.image_location.as_ref() {
                        ui.add(
                            egui::Image::new(format!("wiki://{}", image.name))
                                .max_width(image.width as f32)
                                .max_height(image.height as f32),
                        );
                    }

                    let res = ui.label(&item.wiki_data.name);

                    if let Some(text) = item.wiki_data.tooltip.as_ref() {
                        res.on_hover_text(text.plain());
                    }
                }
            }
        });
    }
}
