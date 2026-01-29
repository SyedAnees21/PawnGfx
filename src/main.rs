use crate::{
    camera::Camera,
    math::{Matrix4, Vector3},
};
use std::sync::Arc;
use winit::event_loop::EventLoopWindowTarget;

mod camera;
mod draw;
mod input;
mod math;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window = Arc::new(
        winit::window::WindowBuilder::new()
            .with_title("BareGFX")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 800.0))
            .build(&event_loop)
            .unwrap(),
    );

    let mut framebuffer = pixels::Pixels::new(
        800,
        800,
        pixels::SurfaceTexture::new(800, 800, window.clone()),
    )
    .unwrap();

    let mut depth_buffer = vec![f64::INFINITY; 800 * 800];

    let mut camera = Camera::new(Vector3::new(0.0, 0.0, 5.0));
    let mut rotation = Vector3::new(0.0, 0.0, 0.0);

    event_loop
        .run(move |e, h| match e {
            winit::event::Event::WindowEvent { event, .. } => handle_window_event(
                &event,
                &mut framebuffer,
                &mut depth_buffer,
                &mut rotation,
                &mut camera,
                h,
            ),
            winit::event::Event::AboutToWait => window.request_redraw(),

            _ => {}
        })
        .unwrap();
}

fn handle_window_event(
    event: &winit::event::WindowEvent,
    framebuffer: &mut pixels::Pixels,
    depth_buffer: &mut Vec<f64>,
    rotation: &mut Vector3,
    camera: &mut Camera,
    handler: &EventLoopWindowTarget<()>,
) {
    match event {
        winit::event::WindowEvent::KeyboardInput { event, .. } => {
            input::process_keyboard_input(event, camera, rotation);
        }
        winit::event::WindowEvent::MouseInput { .. } => {
            input::process_mouse_input(event);
        }
        winit::event::WindowEvent::Resized(size) => {
            framebuffer.resize_surface(size.width, size.height).unwrap();
            framebuffer.resize_buffer(size.width, size.height).unwrap();
            depth_buffer.resize((size.width * size.height) as usize, f64::INFINITY);
        }
        winit::event::WindowEvent::RedrawRequested => {
            let s_width = framebuffer.texture().width();
            let s_height = framebuffer.texture().height();
            let aspect = s_width as f64 / s_height as f64;

            let frame = framebuffer.frame_mut();
            frame.fill(0);

            let model = Matrix4::rotation_matrix(*rotation);
            let view = camera.get_view_matrix();
            let projection = Matrix4::perspective_matrix(90.0_f64.to_radians(), aspect, 0.1, 100.0);
            let mvp = projection * view * model;

            draw::draw_cube(frame, mvp, s_width as f64, s_height as f64);

            framebuffer.render().unwrap();
        }
        winit::event::WindowEvent::CloseRequested => handler.exit(),
        _ => {}
    }
}
