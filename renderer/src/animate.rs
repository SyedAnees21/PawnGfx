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
