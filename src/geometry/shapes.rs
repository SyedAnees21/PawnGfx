use crate::{
    geometry::{Mesh, Normals},
    math::Vector3,
    scene::{Object, Transform},
};

pub const CUBE_VERTS: [Vector3; 8] = [
    Vector3::new(-1.0, -1.0, -1.0), // 0
    Vector3::new(1.0, -1.0, -1.0),  // 1
    Vector3::new(1.0, 1.0, -1.0),   // 2
    Vector3::new(-1.0, 1.0, -1.0),  // 3
    Vector3::new(-1.0, -1.0, 1.0),  // 4
    Vector3::new(1.0, -1.0, 1.0),   // 5
    Vector3::new(1.0, 1.0, 1.0),    // 6
    Vector3::new(-1.0, 1.0, 1.0),   // 7
];

const EDGES: [(usize, usize); 12] = [
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 0),
    (4, 5),
    (5, 6),
    (6, 7),
    (7, 4),
    (0, 4),
    (1, 5),
    (2, 6),
    (3, 7),
];

pub const CUBE_TRIS: [usize; 36] = [
    // FRONT  (-Z)
    0, 2, 1, 0, 3, 2, // BACK   (+Z)
    4, 5, 6, 4, 6, 7, // LEFT   (-X)
    0, 7, 3, 0, 4, 7, // RIGHT  (+X)
    1, 2, 6, 1, 6, 5, // TOP    (+Y)
    3, 7, 6, 3, 6, 2, // BOTTOM (-Y)
    0, 1, 5, 0, 5, 4,
];

pub struct Cube(pub Object);

impl Cube {
    pub fn new(size: f64) -> Self {
        let c_mesh = Mesh::from_vertices_faces(CUBE_VERTS.into(), CUBE_TRIS.into());

        let transform = Transform {
            scale: Vector3::splat(size),
            ..Default::default()
        };

        Self(Object {
            mesh: c_mesh,
            transform,
        })
    }
}
