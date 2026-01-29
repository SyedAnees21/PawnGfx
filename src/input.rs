use crate::{camera, math::Vector3};
use winit::{
    event::KeyEvent,
    keyboard::{self, PhysicalKey},
};

pub fn process_keyboard_input(
    event: &KeyEvent,
    camera: &mut camera::Camera,
    rotation: &mut Vector3,
) {
    if let PhysicalKey::Code(key_code) = event.physical_key {
        let camera_speed = 0.1;
        match key_code {
            keyboard::KeyCode::KeyW => camera.move_forward(camera_speed),
            keyboard::KeyCode::KeyS => camera.move_forward(-camera_speed),
            keyboard::KeyCode::KeyA => camera.move_right(-camera_speed),
            keyboard::KeyCode::KeyD => camera.move_right(camera_speed),
            keyboard::KeyCode::ArrowDown => rotation.x += 0.9,
            keyboard::KeyCode::ArrowUp => rotation.x -= 0.9,
            keyboard::KeyCode::ArrowRight => rotation.y += 0.9,
            keyboard::KeyCode::ArrowLeft => rotation.y -= 0.9,
            _ => {}
        }
    }
}

pub fn process_mouse_input(event: &winit::event::WindowEvent) {
    match event {
        winit::event::WindowEvent::MouseInput { state, button, .. } => match button {
            winit::event::MouseButton::Left => {
                if *state == winit::event::ElementState::Pressed {
                    println!("Left mouse button pressed");
                } else {
                    println!("Left mouse button released");
                }
            }
            winit::event::MouseButton::Right => {
                if *state == winit::event::ElementState::Pressed {
                    println!("Right mouse button pressed");
                } else {
                    println!("Right mouse button released");
                }
            }
            _ => {}
        },
        _ => {}
    }
}
