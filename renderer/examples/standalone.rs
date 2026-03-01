use {
	pcore::error::PResult,
	pixels::{Pixels, SurfaceTexture},
	prenderer::render,
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

	let scene = pscene::global::Scene::default();
	let renderer = render::Renderer::new(size.width, size.height);
	let input = InputState::default();

	// let mut engine = Engine::new(scene, renderer,
	// input);
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
