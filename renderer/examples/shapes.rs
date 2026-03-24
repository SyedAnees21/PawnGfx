use {
	pcore::{color::Color, error::PResult, math::Vector3},
	pixels::{Pixels, SurfaceTexture},
	prenderer::render,
	pscene::{
		assets::registry::{AssetRegistry, MaterialHandle, MeshHandle},
		global::Scene,
		light::Light,
		material::Material,
		model::Model,
	},
	std::sync::Arc,
	winit::{
		event::{DeviceEvent, Event, WindowEvent},
		event_loop::{EventLoop, EventLoopWindowTarget},
		window::{Window, WindowBuilder},
	},
};

include!("utils/fps.rs");
include!("utils/input.rs");

struct Engine<'f> {
	scene: Scene,
	fps: FPSCounter,
	renderer: render::Renderer,
	window: Arc<Window>,
	frame: Pixels<'f>,
	input: InputState,
}

impl<'f> Engine<'f> {
	fn start_internal_loop(
		&mut self,
		event: Event<()>,
		handler: &EventLoopWindowTarget<()>,
	) -> PResult<()> {
		match event {
			Event::WindowEvent { event, .. } => handle_internal_events(
				event,
				&mut self.scene,
				&mut self.renderer,
				&mut self.frame,
				&mut self.input,
				handler,
			)?,
			Event::AboutToWait => {
				update_scene(&mut self.scene, &self.input);
				self.fps.update();
				self.window.request_redraw();
			}
			Event::DeviceEvent {
				event: DeviceEvent::MouseMotion { delta },
				..
			} => {
				self.input.mouse_delta.0 += delta.0 as f32;
				self.input.mouse_delta.1 += delta.1 as f32;
			}
			_ => {}
		}

		Ok(())
	}
}

fn handle_internal_events(
	event: WindowEvent,
	scene: &mut Scene,
	renderer: &mut render::Renderer,
	frame: &mut Pixels<'_>,
	ism: &mut InputState,
	handler: &EventLoopWindowTarget<()>,
) -> PResult<()> {
	match event {
		WindowEvent::KeyboardInput { .. } | WindowEvent::MouseInput { .. } => {
			read_inputs(ism, &event);
		}
		WindowEvent::Resized(size) => {
			frame.resize_surface(size.width, size.height)?;
			frame.resize_buffer(size.width, size.height)?;
			renderer.resize(size.width, size.height);
		}
		WindowEvent::RedrawRequested => {
			renderer.render(scene, frame.frame_mut()).unwrap();
			frame.render()?;
			ism.reset();
		}
		WindowEvent::CloseRequested => handler.exit(),
		_ => {}
	}

	Ok(())
}

fn update_scene(scene: &mut Scene, input: &InputState) {
	scene.camera.apply_inputs(input);
}

fn build_matte_material(scene: &mut Scene) -> MaterialHandle {
	let mut matte = Material::default();
	matte.set_shininess(200.0);
	matte.specular = Color::new_rgb_splat(1.0);
	matte.diffuse = Color::new_rgb_splat(0.2);
	scene.assets.insert_material(matte)
}

fn main() -> PResult<()> {
	let event_loop = EventLoop::new()?;

	let window_builder = WindowBuilder::new();
	let inner = window_builder
		.with_maximized(true)
		.with_title("PawnGFX Shapes")
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

	let sphere = pcore::geometry::generate_sphere(1.25, 48, 32);
	let cube = pcore::geometry::generate_cube(1.6);
	let pyramid = pcore::geometry::generate_pyramid(1.6, 1.6, 2.0);
	let cone = pcore::geometry::generate_cone(1.0, 2.0, 64);
	let frustum = pcore::geometry::generate_frustum(1.8, 1.8, 1.0, 1.0, 2.0);
	let plane = pcore::geometry::generate_plane(12.0, 12.0);

	let mut scene = Scene {
		camera: pscene::camera::Camera::new(Vector3::new(0.0, 0.0, 5.0)),
		light: Light::default(),
		assets: AssetRegistry::default(),
		objects: Vec::new(),
	};

	let h_matte = build_matte_material(&mut scene);

	let h_sphere = scene.assets.insert_mesh(sphere);
	let h_cube = scene.assets.insert_mesh(cube);
	let h_pyramid = scene.assets.insert_mesh(pyramid);
	let h_cone = scene.assets.insert_mesh(cone);
	let h_frustum = scene.assets.insert_mesh(frustum);
	let h_plane = scene.assets.insert_mesh(plane);

	let make_object = |mesh: MeshHandle, pos: Vector3| {
		let mut obj = Object::from_model(Model {
			material: h_matte,
			mesh,
		});
		obj.transform.position = pos;
		obj
	};

	// Ground plane
	scene
		.objects
		.push(make_object(h_plane, Vector3::new(0.0, -1.25, 0.0)));

	// Scattered layout across the plane (centers sit on the plane)
	scene
		.objects
		.push(make_object(h_sphere, Vector3::new(-4.6, 0.0, -2.8))); // r=1.25
	scene
		.objects
		.push(make_object(h_cube, Vector3::new(-0.8, -0.45, 3.6))); // half=0.8
	scene
		.objects
		.push(make_object(h_pyramid, Vector3::new(3.6, -0.25, -0.6))); // half=1.0
	scene
		.objects
		.push(make_object(h_cone, Vector3::new(4.2, -0.25, 2.4))); // half=1.0
	scene
		.objects
		.push(make_object(h_frustum, Vector3::new(0.6, -0.25, -4.0))); // half=1.0

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
