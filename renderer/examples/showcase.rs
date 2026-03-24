use {
	pcore::{color::Color, error::PResult, math::Vector3},
	pixels::{Pixels, SurfaceTexture},
	prenderer::render,
	pscene::{
		assets::{
			registry::{AssetRegistry, MaterialHandle},
		},
		global::Scene,
		light::Light,
		material::Material,
		model::Model,
		texture::{AlbedoMap as Albedo, NormalMap, Wrap},
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
	Matte,
	Metallic,
	Albedo,
	Normal,
}

impl Mode {
	fn index(self) -> usize {
		match self {
			Mode::Matte => 0,
			Mode::Metallic => 1,
			Mode::Albedo => 2,
			Mode::Normal => 3,
		}
	}
}

struct ShowcaseState {
	materials: [MaterialHandle; 4],
	active: Mode,
	tab_down: bool,
}

impl ShowcaseState {
	fn new(materials: [MaterialHandle; 4]) -> Self {
		Self {
			materials,
			active: Mode::Matte,
			tab_down: false,
		}
	}

	fn update(&mut self, input: &InputState, scene: &mut Scene) {
		let tab_now = input.is_pressed(Keys::Tab);
		if tab_now && !self.tab_down {
			self.active = match self.active {
				Mode::Matte => Mode::Metallic,
				Mode::Metallic => Mode::Albedo,
				Mode::Albedo => Mode::Normal,
				Mode::Normal => Mode::Matte,
			};
			if let Some(object) = scene.objects.get_mut(0) {
				object.model.material = self.materials[self.active.index()];
			}
		}
		self.tab_down = tab_now;
	}
}

struct Engine<'f> {
	scene: Scene,
	fps: FPSCounter,
	renderer: render::Renderer,
	window: Arc<Window>,
	frame: Pixels<'f>,
	input: InputState,
	showcase: ShowcaseState,
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
				update_scene(&mut self.scene, &self.input, &mut self.showcase);
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

fn update_scene(
	scene: &mut Scene,
	input: &InputState,
	showcase: &mut ShowcaseState,
) {
	showcase.update(input, scene);
	scene.camera.apply_inputs(input);
	for object in scene.objects.iter_mut() {
		object.apply_inputs(input)
	}
}

fn main() -> PResult<()> {
	let event_loop = EventLoop::new()?;

	let window_builder = WindowBuilder::new();
	let inner = window_builder
		.with_maximized(true)
		.with_title("PawnGFX Showcase")
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
	let sphere_mesh = pcore::geometry::generate_sphere(2.0, 64, 48);

	let albedo =
		Albedo::load("./assets/texture/Checker-Texture.png", Wrap::Mirror).unwrap();

	let normal =
		NormalMap::load("./assets/texture/stones-normal.png", Wrap::Repeat)
			.unwrap();

	let mut scene = Scene {
		camera: pscene::camera::Camera::new(Vector3::new(0.0, 0.0, 5.0)),
		light: Light::default(),
		assets: AssetRegistry::default(),
		objects: Vec::new(),
	};

	let h_albedo = scene.assets.insert_albedo(albedo);
	let h_normal = scene.assets.insert_normal(normal);
	let h_mesh = scene.assets.insert_mesh(sphere_mesh);

	let mut matte = Material::default();
	matte.set_shininess(8.0);
	matte.specular = Color::BLACK;
	matte.set_specular(0.0);
	let h_matte = scene.assets.insert_material(matte);

	let mut metallic = Material::default();
	metallic.set_shininess(200.0);
	metallic.specular = Color::new_rgb_splat(1.0);
	metallic.diffuse = Color::new_rgb_splat(0.2);
	metallic.set_specular(1.0);
	let h_metallic = scene.assets.insert_material(metallic);

	let mut albedo_only = Material::default();
	albedo_only.set_shininess(200.0);
	albedo_only.specular = Color::new_rgb_splat(1.0);
	albedo_only.diffuse = Color::new_rgb_splat(0.2);
	albedo_only.set_specular(1.0);
	albedo_only.set_albedo(h_albedo);
	let h_albedo_only = scene.assets.insert_material(albedo_only);

	let mut normal_mapped = Material::default();
	normal_mapped.set_shininess(200.0);
	normal_mapped.specular = Color::new_rgb_splat(1.0);
	normal_mapped.diffuse = Color::new_rgb_splat(0.2);
	normal_mapped.set_specular(1.0);
	normal_mapped.set_albedo(h_albedo);
	normal_mapped.set_normal_map(h_normal);
	let h_normal_mapped = scene.assets.insert_material(normal_mapped);

	let model = Model {
		material: h_matte,
		mesh: h_mesh,
	};

	scene.objects.push(Object::from_model(model));

	let showcase =
		ShowcaseState::new([h_matte, h_metallic, h_albedo_only, h_normal_mapped]);

	let mut engine = Engine {
		scene,
		fps: FPSCounter::default(),
		renderer,
		window,
		frame,
		input,
		showcase,
	};

	event_loop.run(move |event, handler| {
		if let Err(err) = engine.start_internal_loop(event, handler) {
			eprintln!("Exiting with error {err}")
		}
	})?;

	Ok(())
}
