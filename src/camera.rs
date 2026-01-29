use crate::math::{Matrix4, Vector3};

pub struct Camera {
    pub position: Vector3,
    pub yaw: f64,
    pub pitch: f64,
    pub up: Vector3,
    pub right: Vector3,
    pub forward: Vector3,
}

impl Camera {
    pub fn new(position: Vector3) -> Self {
        let mut cam = Camera {
            position,
            yaw: -90.0,
            pitch: 0.0,
            up: Vector3::ZERO,
            right: Vector3::ZERO,
            forward: Vector3::ZERO,
        };

        cam.update_vectors();
        cam
    }

    pub fn move_forward(&mut self, delta: f64) {
        self.position += self.forward * delta;
    }

    pub fn move_right(&mut self, delta: f64) {
        self.position += self.right * delta;
    }

    pub fn move_up(&mut self, delta: f64) {
        self.position.y += delta;
    }

    pub fn rotate(&mut self, delta_yaw: f64, delta_pitch: f64) {
        self.yaw += delta_yaw;
        self.pitch += delta_pitch;

        self.pitch = self.pitch.clamp(-89.0, 89.0);
        self.update_vectors();
    }

    pub fn update_vectors(&mut self) {
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();

        self.forward = Vector3 {
            x: yaw_rad.cos() * pitch_rad.cos(),
            y: pitch_rad.sin(),
            z: yaw_rad.sin() * pitch_rad.cos(),
        }
        .normalize();

        let world_up = Vector3::UNIT_Y;

        self.right = self.forward.cross(&world_up).normalize();
        self.up = self.right.cross(&self.forward).normalize();
    }

    pub fn get_view_matrix(&self) -> crate::math::Matrix4 {
        let r = self.right;
        let u = self.up;
        let f = self.forward;
        let p = self.position;

        Matrix4 {
            data: [
                [r.x, r.y, r.z, -r.dot(&p)],
                [u.x, u.y, u.z, -u.dot(&p)],
                [-f.x, -f.y, -f.z, f.dot(&p)],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}
