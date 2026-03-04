use std::sync::Arc;

use egui::{Align2, UiBuilder, Vec2, vec2};
use serde::{Deserialize, Serialize};

use self::pages::tree::Node;

mod pages {
    pub mod tree;
}

mod storage_keys {
    pub const UI_STATE: &str = "ui_state";
    pub const TREE_ROOT: &str = "tree_root";
}

pub struct TerratreeApp {
    ui_state: UiState,
    tree_root: Node,
    dragging: Dragging,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
struct UiState {
    item_search: String,
}

#[derive(Default)]
enum Dragging {
    #[default]
    None,
    Item(&'static wiki_data::Item),
}

impl TerratreeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        egui_extras::install_image_loaders(&cc.egui_ctx);

        cc.egui_ctx
            .add_bytes_loader(Arc::new(crate::wiki_img::Loader::new()));

        cc.egui_ctx
            .all_styles_mut(|s| s.interaction.selectable_labels = false);

        let ui_state = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, storage_keys::UI_STATE))
            .unwrap_or_default();

        let tree_root = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, storage_keys::TREE_ROOT))
            .unwrap_or(Node::from_name("Copper Pickaxe").unwrap());

        Self {
            ui_state,
            tree_root,
            dragging: Dragging::default(),
        }
    }
}

impl eframe::App for TerratreeApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, storage_keys::UI_STATE, &self.ui_state);
        eframe::set_value(storage, storage_keys::TREE_ROOT, &self.tree_root);
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

        pages::tree::ui(ctx, self);

        match self.dragging {
            Dragging::None => return,
            Dragging::Item(item) => {
                if let Some(image) = item.image_location.as_ref() {
                    let align = Align2::CENTER_CENTER;
                    let spacing = 0.;
                    let item_size = vec2(32., 32.);

                    let pointer_pos = ctx.pointer_interact_pos().unwrap_or_default();
                    let item_rect = align
                        .anchor_size(pointer_pos, item_size + Vec2::splat(spacing * 2.))
                        .shrink(spacing);

                    egui::Ui::new(
                        ctx.clone(),
                        egui::Id::new("dragging"),
                        UiBuilder::new()
                            .max_rect(item_rect)
                            .layout(egui::Layout::centered_and_justified(egui::Direction::TopDown)),
                    )
                    .add(egui::Image::new(format!("wiki://{}", image.name)));
                }
            }
        }

        if ctx.dragged_id().is_none() {
            self.dragging = Dragging::None;
        } else {
            ctx.output_mut(|o| o.cursor_icon = egui::CursorIcon::Grab);
        }
    }
}
