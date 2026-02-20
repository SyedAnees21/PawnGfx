use eframe::egui;
use rfd::FileDialog;

struct SceneObject {
    name: String,
    obj_type: String,
}

struct RendererApp {
    // UI State
    scene_objects: Vec<SceneObject>,
    // Your renderer components
    width: usize,
    height: usize,
    framebuffer: Vec<u8>,
    texture: Option<egui::TextureHandle>,
}

impl RendererApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            scene_objects: vec![
                SceneObject {
                    name: "Main Camera".into(),
                    obj_type: "Camera".to_string(),
                },
                SceneObject {
                    name: "Point Light".into(),
                    obj_type: "Light".to_string(),
                },
                SceneObject {
                    name: "Cube".into(),
                    obj_type: "Mesh".to_string(),
                },
            ],
            width: 800,
            height: 600,
            framebuffer: vec![0; 800 * 600 * 4],
            texture: None,
        }
    }
}

impl eframe::App for RendererApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. TOP-LEFT: FILE LOADING BUTTON
        egui::Area::new(egui::Id::new("file_loader"))
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(10.0, 10.0))
            .show(ctx, |ui| {
                if ui.button("📂 Load Mesh").clicked() {
                    if let Some(path) = FileDialog::new().add_filter("obj", &["obj"]).pick_file() {
                        println!("Loading: {:?}", path);
                        // self.renderer.load(path);
                    }
                }
            });

        // 2. TOP-MIDDLE: FLOATING CONTROL BUTTONS
        egui::Area::new(egui::Id::new("top_controls"))
            .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 10.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.style_mut().spacing.item_spacing.x = 5.0;
                    if ui.button("⟲ Reset").clicked() { /* Reset Camera */ }
                    if ui.button("▶ Play").clicked() { /* Start Anim */ }
                    if ui.button("⚙ Stats").clicked() { /* Toggle Info */ }
                });
            });

        // 3. RIGHT PANEL: SCENE HIERARCHY
        egui::SidePanel::right("scene_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Scene Objects");
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for obj in &self.scene_objects {
                        let icon = match obj.obj_type.as_str() {
                            "Camera" => "📷",
                            "Light" => "💡",
                            _ => "📦",
                        };
                        ui.selectable_label(false, format!("{} {}", icon, obj.name));
                    }
                });
            });

        // 4. CENTRAL PANEL: THE RENDERER VIEWPORT
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(15, 15, 15)))
            .show(ctx, |ui| {
                // Here we would run the renderer:
                // self.renderer.render(&mut self.framebuffer);

                // For now, let's just display a placeholder image
                let image =
                    egui::ColorImage::from_rgba_unmultiplied([self.width, self.height], &self.framebuffer);
                let texture = self
                    .texture
                    .get_or_insert_with(|| ctx.load_texture("viewport", image.clone(), Default::default()));
                texture.set(image, Default::default());

                ui.centered_and_justified(|ui| {
                    ui.image(&*texture);
                });
            });

        // Keep the UI responsive for real-time rendering
        ctx.request_repaint();
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "PawnGFX",
        native_options,
        Box::new(|cc| Box::new(RendererApp::new(cc))),
    ).unwrap();
}
