use {
	crate::{state::EditorState, ui::UiState},
	eframe::egui,
};

pub fn show(
	ctx: &egui::Context,
	state: &mut EditorState,
	ui_state: &mut UiState,
) {
	egui::CentralPanel::default()
		.frame(egui::Frame::none().fill(egui::Color32::from_rgb(15, 15, 15)))
		.show(ctx, |ui| {
			let image = egui::ColorImage::from_rgba_unmultiplied(
				[state.viewport.width, state.viewport.height],
				&state.viewport.framebuffer,
			);

			let texture = ui_state.viewport_texture.get_or_insert_with(|| {
				ui.ctx()
					.load_texture("viewport", image.clone(), Default::default())
			});
			texture.set(image, Default::default());

			ui.centered_and_justified(|ui| {
				ui.image(&*texture);
			});
		});
}
