use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::EventLoopWindowTarget,
};

use crate::{
    input::{self, InputState},
    render::Renderer,
    scene::Scene,
};

pub struct Engine<'a> {
    scene: Scene,
    renderer: Renderer<'a>,
    input: InputState,
}

impl<'a> Engine<'a> {
    pub fn new(scene: Scene, renderer: Renderer<'a>, input: InputState) -> Self {
        Self {
            scene,
            renderer,
            input,
        }
    }

    pub fn start_internal_loop(
        &mut self,
        event: Event<()>,
        handler: &EventLoopWindowTarget<()>,
    ) {
        match event {
            Event::WindowEvent { event, .. } => handle_internal_events(
                event,
                &mut self.scene,
                &mut self.renderer,
                &mut self.input,
                handler,
            ),
            Event::AboutToWait => {
                let animator = &mut self.scene.animator;
                let camera = &mut self.scene.camera;

                if !animator.is_complete() {
                    camera.position = animator.step(0.005);
                } else {
                    self.input.apply_inputs(
                        &mut self.scene.camera,
                        &mut self.scene.object.transform.rotation,
                    );
                }
                

                self.renderer.get_window().request_redraw()
            }
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    self.input.mouse_delta.0 += delta.0;
                    self.input.mouse_delta.1 += delta.1;
                }
                _ => {}
            },

            _ => {}
        }
    }
}

fn handle_internal_events<'a>(
    event: WindowEvent,
    scene: &mut Scene,
    renderer: &mut Renderer<'a>,
    ism: &mut InputState,
    handler: &EventLoopWindowTarget<()>,
) {
    match event {
        WindowEvent::KeyboardInput { .. } | WindowEvent::MouseInput { .. } => {
            input::read_inputs(ism, &event);
        }
        WindowEvent::Resized(size) => {
            renderer.resize_buffers(size.width, size.height);
        }
        WindowEvent::RedrawRequested => {
            renderer.render(scene);
            ism.reset();
        }
        WindowEvent::CloseRequested => handler.exit(),
        _ => {}
    }
}
