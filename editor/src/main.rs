mod app;
mod state;
mod ui;

use app::EditorApp;

fn main() {
	let native_options = eframe::NativeOptions::default();
	eframe::run_native(
		"PawnGFX",
		native_options,
		Box::new(|cc| Box::new(EditorApp::new(cc))),
	)
	.unwrap();
}
