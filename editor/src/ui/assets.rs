use eframe::egui;
use rfd::FileDialog;

use crate::{state::{AssetKind, EditorState}, ui::UiState};

pub fn show(ctx: &egui::Context, state: &mut EditorState, ui_state: &mut UiState) {
    egui::Area::new(egui::Id::new("assets_fab"))
        .anchor(egui::Align2::LEFT_TOP, egui::vec2(8.0, 8.0))
        .show(ctx, |ui| {
            egui::Frame::window(&ctx.style()).show(ui, |ui| {
                if ui.button("+").on_hover_text("Assets").clicked() {
                    ui_state.show_assets = !ui_state.show_assets;
                }
            });
        });

    if !ui_state.show_assets {
        return;
    }

    egui::Window::new("Assets")
        .id(egui::Id::new("assets_window"))
        .default_pos(egui::pos2(10.0, 40.0))
        .resizable(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("M").on_hover_text("Load Mesh").clicked() {
                    if let Some(path) = FileDialog::new().add_filter("obj", &["obj"]).pick_file() {
                        let _ = state.assets.load_mesh(path);
                    }
                }
                if ui.button("A").on_hover_text("Load Albedo").clicked() {
                    if let Some(path) = FileDialog::new().add_filter("images", &["png", "jpg", "jpeg"]).pick_file() {
                        let _ = state.assets.load_albedo(path);
                    }
                }
                if ui.button("N").on_hover_text("Load Normal").clicked() {
                    if let Some(path) = FileDialog::new().add_filter("images", &["png", "jpg", "jpeg"]).pick_file() {
                        let _ = state.assets.load_normal(path);
                    }
                }
            });

            if let Some(err) = state.assets.last_error() {
                ui.separator();
                ui.colored_label(egui::Color32::LIGHT_RED, format!("Load error: {err}"));
            }

            ui.separator();
            let mut assign_action = None;

            egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
                for asset in state.assets.records() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{:?}", asset.kind));
                        ui.label(&asset.name);
                        ui.monospace(asset.path.display().to_string());
                        if state.selected.is_some() {
                            let assign_label = match asset.kind {
                                AssetKind::Mesh => "M",
                                AssetKind::Albedo => "A",
                                AssetKind::Normal => "N",
                            };
                            let hint = match asset.kind {
                                AssetKind::Mesh => "Assign Mesh",
                                AssetKind::Albedo => "Assign Albedo",
                                AssetKind::Normal => "Assign Normal",
                            };
                            if ui.button(assign_label).on_hover_text(hint).clicked() {
                                assign_action = Some((asset.kind, asset.id));
                            }
                        }
                    });
                }
            });

            if let Some((kind, id)) = assign_action {
                match kind {
                    AssetKind::Mesh => state.assign_mesh_to_selected(id),
                    AssetKind::Albedo => state.assign_albedo_to_selected(id),
                    AssetKind::Normal => state.assign_normal_to_selected(id),
                }
            }
        });
}
