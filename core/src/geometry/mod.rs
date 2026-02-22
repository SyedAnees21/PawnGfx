mod mesh;
mod triangle;
mod vertex;

use std::ops::{Add, Mul, Sub};

use image::Rgb;
pub use mesh::*;
pub use triangle::*;
pub use vertex::*;

use crate::math::{Vector2, Vector3};

pub type Idx = usize;

/// Vertex Index
pub type VIdx = usize;
/// Vertex Normal Index
pub type NIdx = usize;
/// Vertex Texture ( uv ) Index
pub type TIdx = usize;

pub type Vertices = Vec<Vector3>;
pub type Normals = Vec<Vector3>;

pub type UV = Vector2;
pub type Vertex = Vector3;
pub type Normal = Vector3;
pub type Tangent = Vector3;
pub type BiTangent = Vector3;

trait Arithmetic: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self>
where
    Self: Sized,
{
}

impl<T> Arithmetic for T where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Sized {}

pub fn edge_function(v0: Vector2, v1: Vector2, p: Vector2) -> f64 {
    (p.x - v0.x) * (v1.y - v0.y) - (p.y - v0.y) * (v1.x - v0.x)
}

pub fn bounding_rect(v0: Vector2, v1: Vector2, v2: Vector2) -> (Vector2, Vector2) {
    let min_x = v0.x.min(v1.x.min(v2.x)).floor();
    let min_y = v0.y.min(v1.y.min(v2.y)).floor();
    let max_x = v0.x.max(v1.x.max(v2.x)).ceil();
    let max_y = v0.y.max(v1.y.max(v2.y)).ceil();

    (Vector2::new(min_x, min_y), Vector2::new(max_x, max_y))
}

impl From<Rgb<u8>> for Normal {
    fn from(value: Rgb<u8>) -> Self {
        Normal::new(
            value[0] as f64 / 255.0 * 2.0 - 1.0,
            value[1] as f64 / 255.0 * 2.0 - 1.0,
            value[2] as f64 / 255.0 * 2.0 - 1.0,
        )
    }
}