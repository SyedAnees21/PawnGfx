use eframe::egui;

use crate::state::EditorState;

const SCENE_WIDTH: f32 = 220.0;
const GAP: f32 = 8.0;

pub fn show(ctx: &egui::Context, state: &mut EditorState) {
    egui::Area::new(egui::Id::new("scene_panel_floating"))
        .anchor(egui::Align2::RIGHT_CENTER, egui::vec2(-GAP, 0.0))
        .show(ctx, |ui| {
            egui::Frame::window(&ctx.style()).show(ui, |ui| {
                ui.set_width(SCENE_WIDTH);
                ui.heading("Scene");
                ui.separator();

                let mut new_selected = None;

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for obj in state.scene_objects.iter() {
                        let icon = match obj.obj_type.as_str() {
                            "Camera" => "[C]",
                            "Light" => "[L]",
                            _ => "[M]",
                        };
                        let selected = state.selected == Some(obj.id);
                        if ui
                            .selectable_label(selected, format!("{} {}", icon, obj.name))
                            .clicked()
                        {
                            new_selected = Some(obj.id);
                        }
                    }
                });

                if let Some(id) = new_selected {
                    state.select(id);
                }
            });
        });
}

pub fn scene_panel_width() -> f32 {
    SCENE_WIDTH
}
