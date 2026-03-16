use {
	pcore::{error::PResult, math::Vector3},
	pixels::{Pixels, SurfaceTexture},
	prenderer::render,
	pscene::{
		assets::{load_mesh_file, registry::AssetRegistry},
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
	let sphere_mesh = load_mesh_file("./assets/meshes/sphere-local.obj").unwrap();

	let albedo =
		Albedo::load("./assets/texture/Checker-Texture.png", Wrap::Mirror)
			.unwrap();

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
	let h_normal = scene.assets.insert_normal(normal);

	let mut material = Material::default();
	material.set_albedo(h_albedo);
	material.set_normal_map(h_normal);

	let h_material = scene.assets.insert_material(material);

	let h_cube_mesh = scene.assets.insert_mesh(cube_mesh);
	let h_sphere_mesh = scene.assets.insert_mesh(sphere_mesh);

	let cube_model = Model {
		material: h_material,
		mesh: h_cube_mesh,
	};

	let sphere_model = Model {
		material: h_material,
		mesh: h_sphere_mesh,
	};

	let cube = Object::from_model(cube_model);
	let mut sphere = Object::from_model(sphere_model);

	// Place sphere behind the cube and offset to the right.
	sphere.transform.position = Vector3::new(-3.0, 0.0, -3.0);

	scene.objects.push(cube);
	scene.objects.push(sphere);

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
