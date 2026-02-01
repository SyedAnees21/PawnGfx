mod mesh;
mod shapes;
mod triangle;

pub use mesh::*;
pub use shapes::*;
pub use triangle::*;

use crate::math::Vector2;

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
