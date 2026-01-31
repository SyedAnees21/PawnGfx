use crate::geometry::Mesh;


pub struct Cube {
    pub size: f64,
    // pub mesh: Mesh,
}

impl Cube {
    pub fn new(size: f64) -> Self {
        Self { size }
    }
}