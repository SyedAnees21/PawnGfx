use std::collections::HashSet;
use crate::{camera, math::Vector3};
use winit::{
    event::{ElementState, KeyEvent},
    keyboard::PhysicalKey,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keys {
    W,
    A,
    S,
    D,
    Q,
    E,
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<winit::keyboard::KeyCode> for Keys {
    type Error = ();

    fn try_from(key_code: winit::keyboard::KeyCode) -> Result<Self, Self::Error> {
        match key_code {
            winit::keyboard::KeyCode::KeyW => Ok(Keys::W),
            winit::keyboard::KeyCode::KeyA => Ok(Keys::A),
            winit::keyboard::KeyCode::KeyS => Ok(Keys::S),
            winit::keyboard::KeyCode::KeyD => Ok(Keys::D),
            winit::keyboard::KeyCode::KeyQ => Ok(Keys::Q),
            winit::keyboard::KeyCode::KeyE => Ok(Keys::E),
            winit::keyboard::KeyCode::ArrowUp => Ok(Keys::Up),
            winit::keyboard::KeyCode::ArrowDown => Ok(Keys::Down),
            winit::keyboard::KeyCode::ArrowLeft => Ok(Keys::Left),
            winit::keyboard::KeyCode::ArrowRight => Ok(Keys::Right),
            _ => Err(()),
        }
    }
}

pub struct InputState {
    pub keys_pressed: HashSet<Keys>,
    pub mouse_right: bool,
    pub mouse_left: bool,
    pub mouse_delta: (f64, f64),
    pub mouse_position: (f64, f64),
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            keys_pressed: HashSet::new(),
            mouse_right: false,
            mouse_left: false,
            mouse_delta: (0.0, 0.0),
            mouse_position: (0.0, 0.0),
        }
    }

    pub fn reset(&mut self) {
        self.reset_mouse_delta();
    }

    pub fn reset_mouse_delta(&mut self) {
        self.mouse_delta = (0.0, 0.0);
    }

    pub fn process_keyboard_input(&mut self, event: &KeyEvent) {
        let PhysicalKey::Code(key_code) = event.physical_key else {
            return;
        };
        let Ok(key) = Keys::try_from(key_code) else {
            return;
        };

        match event.state {
            winit::event::ElementState::Pressed => {
                self.keys_pressed.insert(key);
            }
            winit::event::ElementState::Released => {
                self.keys_pressed.remove(&key);
            }
        }
    }

    pub fn process_mouse_input(
        &mut self,
        state: &ElementState,
        button: &winit::event::MouseButton,
    ) {
        match button {
            winit::event::MouseButton::Left => {
                self.mouse_left = *state == winit::event::ElementState::Pressed;
            }
            winit::event::MouseButton::Right => {
                self.mouse_right = *state == winit::event::ElementState::Pressed;
            }
            _ => {}
        }
    }

    pub fn apply_inputs(&mut self, camera: &mut camera::Camera, rotation: &mut Vector3) {
        let camera_speed = 0.1;
        for key in &self.keys_pressed {
            match key {
                Keys::W => camera.move_forward(camera_speed),
                Keys::S => camera.move_forward(-camera_speed),
                Keys::A => camera.move_right(-camera_speed),
                Keys::D => camera.move_right(camera_speed),
                Keys::Q => camera.move_up(-camera_speed),
                Keys::E => camera.move_up(camera_speed),
                Keys::Up => rotation.x -= 0.9,
                Keys::Down => rotation.x += 0.9,
                Keys::Left => rotation.y -= 0.9,
                Keys::Right => rotation.y += 0.9,
            }
        }

        if self.mouse_right {
            let (delta_x, delta_y) = self.mouse_delta;
            let sensitivity = 0.1;
            camera.rotate(delta_x * sensitivity, -delta_y * sensitivity);
        }
    }
}

pub fn read_inputs(ism: &mut InputState, event: &winit::event::WindowEvent) {
    match event {
        winit::event::WindowEvent::KeyboardInput { event, .. } => {
            ism.process_keyboard_input(event);
        }
        winit::event::WindowEvent::CursorMoved { position, .. } => {
            ism.mouse_delta.0 += position.x - ism.mouse_position.0;
            ism.mouse_delta.1 += position.y - ism.mouse_position.1;
            ism.mouse_position = (position.x, position.y);
        }
        winit::event::WindowEvent::MouseInput { state, button, .. } => {
            ism.process_mouse_input(state, button)
        }
        _ => {}
    }
}
