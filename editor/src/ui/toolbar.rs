use eframe::egui;

use crate::state::{EditorState, ShapeType, Tool};

pub fn show(ctx: &egui::Context, state: &mut EditorState) {
    egui::Area::new(egui::Id::new("toolbar_floating"))
        .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 8.0))
        .show(ctx, |ui| {
            egui::Frame::window(&ctx.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    tool_button(ui, state, Tool::Select, "[]", "Select");
                    tool_button(ui, state, Tool::Move, "➡️", "Move");
                    tool_button(ui, state, Tool::Rotate, "🔄️", "Rotate");
                    tool_button(ui, state, Tool::Scale, "🔺", "Scale");

                    ui.separator();
                    if ui.button("C").on_hover_text("Add Cube").clicked() {
                        state.add_shape(ShapeType::Cube);
                    }
                    if ui.button("O").on_hover_text("Add Sphere").clicked() {
                        state.add_shape(ShapeType::Sphere);
                    }
                    if ui.button("P").on_hover_text("Add Plane").clicked() {
                        state.add_shape(ShapeType::Plane);
                    }
                });
            });
        });
}

fn tool_button(ui: &mut egui::Ui, state: &mut EditorState, tool: Tool, label: &str, hint: &str) {
    let selected = state.tool == tool;
    if ui
        .selectable_label(selected, label)
        .on_hover_text(hint)
        .clicked()
    {
        state.set_tool(tool);
    }
}
