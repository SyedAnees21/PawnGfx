use {
	crate::{state::EditorState, ui::scene_panel},
	eframe::egui,
};

const INSPECTOR_WIDTH: f32 = 260.0;
const GAP: f32 = 8.0;

pub fn show(ctx: &egui::Context, state: &mut EditorState) {
	let open = state.selected.is_some();
	let t = ctx.animate_bool(egui::Id::new("inspector_open"), open);
	if t <= 0.01 {
		return;
	}

	let width = egui::lerp(0.0..=INSPECTOR_WIDTH, t);
	let anchor_x = -(scene_panel::scene_panel_width() + GAP + GAP);

	egui::Area::new(egui::Id::new("inspector_floating"))
		.anchor(egui::Align2::RIGHT_CENTER, egui::vec2(anchor_x, 0.0))
		.show(ctx, |ui| {
			egui::Frame::window(&ctx.style()).show(ui, |ui| {
				ui.set_width(width);
				if width < 24.0 {
					return;
				}

				ui.heading("Inspector");
				ui.separator();

				let Some(selected) = state.selected else {
					ui.label("No object selected.");
					return;
				};

				let mesh_label = state
					.selected_object()
					.and_then(|obj| obj.mesh_id)
					.and_then(|id| state.assets.record_name(id))
					.unwrap_or("None")
					.to_string();
				let albedo_label = state
					.selected_object()
					.and_then(|obj| obj.albedo_id)
					.and_then(|id| state.assets.record_name(id))
					.unwrap_or("None")
					.to_string();
				let normal_label = state
					.selected_object()
					.and_then(|obj| obj.normal_id)
					.and_then(|id| state.assets.record_name(id))
					.unwrap_or("None")
					.to_string();

				let Some(obj) =
					state.scene_objects.iter_mut().find(|o| o.id == selected)
				else {
					ui.label("Selected object not found.");
					return;
				};

				ui.horizontal(|ui| {
					ui.label("Name:");
					ui.text_edit_singleline(&mut obj.name);
				});

				ui.label(format!("Type: {}", obj.obj_type));
				ui.separator();

				ui.label("Transform");
				vec3_editor(ui, "Position", &mut obj.transform.position);
				vec3_editor(ui, "Rotation", &mut obj.transform.rotation);
				vec3_editor(ui, "Scale", &mut obj.transform.scale);

				ui.separator();
				ui.label("Mesh");
				ui.label(mesh_label);

				ui.separator();
				ui.label("Material / Albedo");
				ui.label(albedo_label);

				ui.separator();
				ui.label("Normal Map");
				ui.label(normal_label);
			});
		});
}

fn vec3_editor(ui: &mut egui::Ui, label: &str, value: &mut [f32; 3]) {
	ui.horizontal(|ui| {
		ui.label(label);
		ui.add(egui::DragValue::new(&mut value[0]).speed(0.1).prefix("x: "));
		ui.add(egui::DragValue::new(&mut value[1]).speed(0.1).prefix("y: "));
		ui.add(egui::DragValue::new(&mut value[2]).speed(0.1).prefix("z: "));
	});
}
