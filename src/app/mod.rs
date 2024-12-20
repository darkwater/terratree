use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::tdata::{self, tree::TDataTree};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct TemplateApp {
    #[serde(skip)]
    pub tree: TDataTree,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            tree: TDataTree::from_tdata(&tdata::Data::load().unwrap()),
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

        egui::CentralPanel::default().show(ctx, |ui| {
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

                    if let Some(text) = item.wiki_data.tooltip.as_ref().map(|t| t.text.as_str()) {
                        res.on_hover_text(text);
                    }
                }
            }
        });
    }
}
