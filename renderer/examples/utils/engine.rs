use prenderer::render::Renderer;
use winit::event::{DeviceEvent, Event, WindowEvent};

use pscene::global::Scene;

pub struct Engine<'f> {
	pub scene: Scene,
	pub fps: FPSCounter,
	pub renderer: Renderer,
	pub window: Arc<Window>,
	pub frame: Pixels<'f>,
	pub input: InputState,
}

impl<'f> Engine<'f> {
	pub fn start_internal_loop(
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
				self.input.mouse_delta.0 += delta.0;
				self.input.mouse_delta.1 += delta.1;
			}
			_ => {}
		}

		Ok(())
	}
}

fn handle_internal_events(
	event: WindowEvent,
	scene: &mut Scene,
	renderer: &mut Renderer,
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
	scene.object.apply_inputs(input);
}
