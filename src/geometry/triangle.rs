use crate::{
    geometry::{edge_function, mesh::Mesh},
    math::{Vector2, Vector3},
};

pub struct Triangles<'a> {
    pub mesh: &'a Mesh,
    pub counter: usize,
}

impl Iterator for Triangles<'_> {
    type Item = (Vector3, Vector3, Vector3);

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter + 2 >= self.mesh.indices.len() {
            return None;
        }

        let (idx0, idx1, idx2) = (
            self.mesh.indices[self.counter],
            self.mesh.indices[self.counter + 1],
            self.mesh.indices[self.counter + 2],
        );

        let vtcs = (
            self.mesh.vertices[idx0],
            self.mesh.vertices[idx1],
            self.mesh.vertices[idx2],
        );

        self.counter += 3;
        Some(vtcs)
    }
}

impl<'a> Triangles<'a> {
    pub fn new(mesh: &'a Mesh) -> Self {
        Self { mesh, counter: 0 }
    }
}

pub fn point_inside_triangle(v0: Vector2, v1: Vector2, v2: Vector2, p: Vector2) -> bool {
    let ef0 = edge_function(v0, v1, p);
    let ef1 = edge_function(v1, v2, p);
    let ef2 = edge_function(v2, v0, p);

    ef0 <= 0.0 && ef1 <= 0.0 && ef2 <= 0.0
}
