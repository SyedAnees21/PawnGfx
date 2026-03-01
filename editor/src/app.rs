use eframe::egui;

use crate::{state::EditorState, ui};

pub struct EditorApp {
    state: EditorState,
    ui_state: ui::UiState,
}

impl EditorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: EditorState::new(),
            ui_state: ui::UiState::new(),
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::render(ctx, &mut self.state, &mut self.ui_state);
        ctx.request_repaint();
    }
}
