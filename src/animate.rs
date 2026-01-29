use crate::math::{Vector3, lerp};

pub struct ProceduralAnimator {
    pub start: Vector3,
    pub end: Vector3,
    pub progress: f64,
}

impl ProceduralAnimator {
    pub fn new(start: Vector3, target: Vector3) -> Self {
        Self {
            start,
            end: target,
            progress: 0.0,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }

    pub fn step(&mut self, delta: f64) -> Vector3 {
        self.progress += delta;
        let t = 1.0 - (1.0 - self.progress).powi(3);

        if self.progress >= 1.0 {
            self.progress = 1.0;
        }
        
        lerp(self.start, self.end, t)
    }
}

pub struct AnimationState {
    pub start_pos: Vector3,
    pub end_pos: Vector3,
    pub start_pitch: f64,
    pub end_pitch: f64,
    pub progress: f64, // 0.0 to 1.0
    pub is_running: bool,
}

impl AnimationState {
    pub fn new(target_pos: Vector3) -> Self {
        Self {
            // Start high up (Y=10) and far back (Z=10)
            start_pos: Vector3::new(15.0, 0.0, 10.0),
            end_pos: target_pos,
            start_pitch: -45.0, // Looking down
            end_pitch: 0.0,     // Looking level
            progress: 0.0,
            is_running: true,
        }
    }
}
