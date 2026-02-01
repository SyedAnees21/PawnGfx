use crate::{
    animate::ProceduralAnimator,
    camera::Camera,
    draw::{CUBE_TRIS, CUBE_VERTS},
    geometry::Mesh,
    input::InputState,
    math::{Matrix4, Vector3, lerp},
};
use core::f64;
use std::sync::Arc;
use winit::event_loop::EventLoopWindowTarget;

mod animate;
mod camera;
mod color;
mod draw;
mod geometry;
mod input;
mod math;
mod raster;

#[cfg(test)]
mod tests;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    let window = Arc::new(
        winit::window::WindowBuilder::new()
            .with_title("BareGFX")
            .with_maximized(true)
            .build(&event_loop)
            .unwrap(),
    );

    let size = window.inner_size();

    let mut framebuffer = pixels::Pixels::new(
        size.width,
        size.height,
        pixels::SurfaceTexture::new(size.width, size.height, window.clone()),
    )
    .unwrap();

    let mut depth_buffer = vec![f64::INFINITY; (size.width * size.height) as usize];
    // FPS camera
    let mut camera = Camera::new(Vector3::new(0.0, 0.0, 5.0));

    // Object rotation
    let mut rotation = Vector3::new(0.0, 0.0, 0.0);

    // input state machine
    let mut ism = input::InputState::default();

    // Just for a juicy intro to this wireframe demo. Its not a serious
    // animation system ;)
    let mut animator =
        ProceduralAnimator::new(Vector3::new(15.0, 0.0, 10.0), Vector3::new(0.0, 0.0, 5.0));

    // Cube mesh
    let cube = Mesh::new(CUBE_VERTS.into(), CUBE_TRIS.into());
    
    // Directional light
    let light = Vector3::new(1.0, 1.0, 2.0).normalize();

    event_loop
        .run(move |e, h| match e {
            winit::event::Event::WindowEvent { event, .. } => handle_window_event(
                &event,
                &mut framebuffer,
                &mut depth_buffer,
                &cube,
                light,
                &mut rotation,
                &mut camera,
                &mut ism,
                h,
            ),
            winit::event::Event::AboutToWait => {
                if !animator.is_complete() {
                    camera.position = animator.step(0.005);
                } else {
                    ism.apply_inputs(&mut camera, &mut rotation);
                }

                window.request_redraw()
            }
            winit::event::Event::DeviceEvent { event, .. } => match event {
                winit::event::DeviceEvent::MouseMotion { delta } => {
                    ism.mouse_delta.0 += delta.0;
                    ism.mouse_delta.1 += delta.1;
                }
                _ => {}
            },

            _ => {}
        })
        .unwrap();
}

fn handle_window_event(
    event: &winit::event::WindowEvent,
    framebuffer: &mut pixels::Pixels,
    depth_buffer: &mut Vec<f64>,
    mesh: &Mesh,
    light: Vector3,
    rotation: &mut Vector3,
    camera: &mut Camera,
    ism: &mut InputState,
    handler: &EventLoopWindowTarget<()>,
) {
    match event {
        winit::event::WindowEvent::KeyboardInput { .. }
        | winit::event::WindowEvent::MouseInput { .. } => {
            input::read_inputs(ism, event);
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
            frame.fill(77);
            depth_buffer.fill(f64::INFINITY);

            let model = Matrix4::rotation_matrix(*rotation);
            let view = camera.get_view_matrix();
            let projection = Matrix4::perspective_matrix(90.0_f64.to_radians(), aspect, 0.1, 100.0);
            let mvp = projection * view * model;

            raster::draw_call(
                frame,
                depth_buffer,
                s_width as i32,
                s_height as i32,
                light,
                mvp,
                mesh.triangles(),
            );

            framebuffer.render().unwrap();
            ism.reset();
        }
        winit::event::WindowEvent::CloseRequested => handler.exit(),
        _ => {}
    }
}
