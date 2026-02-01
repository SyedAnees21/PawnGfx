use crate::math::{Matrix4, Vector3};

const UP: usize = 0;
const RIGHT: usize = 1;
const FORWARD: usize = 2;

pub struct Camera {
    pub position: Vector3,
    pub yaw: f64,
    pub pitch: f64,
    pub basis: [Vector3; 3],
}

impl Camera {
    pub fn new(position: Vector3) -> Self {
        let mut cam = Camera {
            position,
            yaw: -90.0,
            pitch: 0.0,
            basis: [Vector3::ZERO; 3],
        };

        cam.update_vectors();
        cam
    }

    pub fn move_forward(&mut self, delta: f64) {
        self.position += self.basis[FORWARD] * delta;
    }

    pub fn move_right(&mut self, delta: f64) {
        self.position += self.basis[RIGHT] * delta;
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

        self.basis[FORWARD] = Vector3 {
            x: yaw_rad.cos() * pitch_rad.cos(),
            y: pitch_rad.sin(),
            z: yaw_rad.sin() * pitch_rad.cos(),
        }
        .normalize();

        let world_up = Vector3::UNIT_Y;

        self.basis[RIGHT] = self.basis[FORWARD].cross(&world_up).normalize();
        self.basis[UP] = self.basis[RIGHT].cross(&self.basis[FORWARD]).normalize();
    }

    pub fn get_view_matrix(&self) -> crate::math::Matrix4 {
        let r = self.basis[RIGHT];
        let u = self.basis[UP];
        let f = self.basis[FORWARD];
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
