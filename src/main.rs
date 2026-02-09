use crate::{engine::Engine, error::PResult, input::InputState, scene::Scene};

mod animate;
mod color;
mod draw;
mod engine;
mod error;
mod geometry;
mod input;
mod math;
mod raster;
mod render;
mod scene;
mod shaders;

#[cfg(test)]
mod tests;

fn main() -> PResult<()> {
    let event_loop = winit::event_loop::EventLoop::new()?;

    let scene = Scene::initialize_default();
    let renderer = render::initialize_renderer("PawnGFX", 0, 0, true, &event_loop)?;
    let input = InputState::default();

    let mut engine = Engine::new(scene, renderer, input);

    event_loop.run(move |event, handler| {
        if let Err(err) = engine.start_internal_loop(event, handler) {
            eprintln!("Exiting with error {err}")
        }
    })?;

    Ok(())
}
