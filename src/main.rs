use crate::{engine::Engine, input::InputState, scene::Scene};

mod animate;
mod color;
mod draw;
mod engine;
mod geometry;
mod input;
mod math;
mod raster;
mod render;
mod scene;

#[cfg(test)]
mod tests;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    let scene = Scene::initialize_default();
    let renderer = render::initialize_renderer("PawnGFX", 0, 0, true, &event_loop);
    let input = InputState::default();

    let mut engine = Engine::new(scene, renderer, input);

    event_loop
        .run(move |event, handler| engine.start_internal_loop(event, handler))
        .unwrap();
}
