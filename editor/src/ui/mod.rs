mod assets;
mod inspector;
mod scene_panel;
mod toolbar;
mod viewport;

use {crate::state::EditorState, eframe::egui};

pub struct UiState {
	pub viewport_texture: Option<egui::TextureHandle>,
	pub show_assets: bool,
}

impl UiState {
	pub fn new() -> Self {
		Self {
			viewport_texture: None,
			show_assets: true,
		}
	}
}

pub fn render(
	ctx: &egui::Context,
	state: &mut EditorState,
	ui_state: &mut UiState,
) {
	toolbar::show(ctx, state);
	assets::show(ctx, state, ui_state);
	scene_panel::show(ctx, state);
	inspector::show(ctx, state);
	viewport::show(ctx, state, ui_state);
}
