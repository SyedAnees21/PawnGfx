use {
	pcore::{color::Color, error::PResult, math::Vector3},
	pixels::{Pixels, SurfaceTexture},
	prenderer::render,
	pscene::{
		assets::{load_mesh_file, registry::AssetRegistry},
		// color::Color,
		light::Light,
		material::Material,
		model::Model,
		texture::{AlbedoMap as Albedo, NormalMap, Wrap},
	},
	std::sync::Arc,
	winit::{
		event_loop::EventLoopWindowTarget,
		window::{Window, WindowBuilder},
	},
};

include!("utils/engine.rs");
include!("utils/fps.rs");
include!("utils/input.rs");

fn main() -> PResult<()> {
	let event_loop = winit::event_loop::EventLoop::new()?;

	let window_builder = WindowBuilder::new();
	let inner = window_builder
		.with_maximized(true)
		.with_title("PawnGFX Standalone")
		.build(&event_loop)?;

	let window = Arc::new(inner);
	let size = window.inner_size();

	let frame = Pixels::new(
		size.width,
		size.height,
		SurfaceTexture::new(size.width, size.height, window.clone()),
	)?;

	let renderer = render::Renderer::new(size.width, size.height);
	let input = InputState::default();

	let cube_mesh = load_mesh_file("./assets/meshes/cube-local.obj").unwrap();
	let plane_mesh = pcore::geometry::generate_plane(12.0, 12.0);

	let albedo =
		Albedo::load("./assets/texture/Checker-Texture.png", Wrap::Mirror).unwrap();

	let normal =
		NormalMap::load("./assets/texture/stones-normal.png", Wrap::Repeat)
			.unwrap();

	let mut scene = Scene {
		camera: Camera::new(Vector3::new(0.0, 0.0, 5.0)),
		light: Light::default(),
		assets: AssetRegistry::default(),
		objects: Vec::new(),
	};

	let h_albedo = scene.assets.insert_albedo(albedo);
	let h_mesh = scene.assets.insert_mesh(cube_mesh);
	let h_plane_mesh = scene.assets.insert_mesh(plane_mesh);
	let h_normal = scene.assets.insert_normal(normal);

	let mut material = Material::default();
	material.set_shininess(200.0);
	material.specular = Color::new_rgb_splat(1.0);
	material.diffuse = Color::new_rgb_splat(0.2);
	material.set_albedo(h_albedo);
	material.set_normal_map(h_normal);

	let h_material = scene.assets.insert_material(material);

	let mut floor_mat = Material::default();
	floor_mat.set_shininess(8.0);
	floor_mat.specular = Color::BLACK;
	floor_mat.diffuse = Color::new_rgb_splat(0.75);
	let h_floor_mat = scene.assets.insert_material(floor_mat);

	let model = Model {
		material: h_material,
		mesh: h_mesh,
	};

	scene.objects.push(Object::from_model(model));

	let mut floor = Object::from_model(Model {
		material: h_floor_mat,
		mesh: h_plane_mesh,
	});
	floor.transform.position = Vector3::new(0.0, -1.25, 0.0);
	scene.objects.push(floor);

	let mut engine = Engine {
		scene,
		fps: FPSCounter::default(),
		renderer,
		window,
		frame,
		input,
	};

	event_loop.run(move |event, handler| {
		if let Err(err) = engine.start_internal_loop(event, handler) {
			eprintln!("Exiting with error {err}")
		}
	})?;

	Ok(())
}
