use crate::{
    geometry::{Normal, UV, mesh::Mesh},
    math::{Vector2, Vector3},
};

pub struct Triangles<'a> {
    pub mesh: &'a Mesh,
    pub counter: usize,
}

impl Iterator for Triangles<'_> {
    type Item = ([Vector3; 3], [Normal; 3], [UV; 3]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter + 2 >= self.mesh.indices.len() {
            return None;
        }

        let (v0, v1, v2) = (
            self.mesh.indices.v[self.counter],
            self.mesh.indices.v[self.counter + 1],
            self.mesh.indices.v[self.counter + 2],
        );

        let v = [
            self.mesh.vertices[v0],
            self.mesh.vertices[v1],
            self.mesh.vertices[v2],
        ];

        let n = if self.mesh.has_normals() {
            let (n0, n1, n2) = (
                self.mesh.indices.n[self.counter],
                self.mesh.indices.n[self.counter + 1],
                self.mesh.indices.n[self.counter + 2],
            );

            [
                self.mesh.normals[n0],
                self.mesh.normals[n1],
                self.mesh.normals[n2],
            ]
        } else {
            [Normal::default(); 3]
        };

        let uv = if self.mesh.has_uv() {
            let (n0, n1, n2) = (
                self.mesh.indices.t[self.counter],
                self.mesh.indices.t[self.counter + 1],
                self.mesh.indices.t[self.counter + 2],
            );

            [self.mesh.uv[n0], self.mesh.uv[n1], self.mesh.uv[n2]]
        } else {
            [UV::default(); 3]
        };

        self.counter += 3;
        Some((v, n, uv))
    }
}

impl<'a> Triangles<'a> {
    pub fn new(mesh: &'a Mesh) -> Self {
        Self { mesh, counter: 0 }
    }
}

#[inline(always)]
pub fn edge_function(v0: Vector2, v1: Vector2, p: Vector2) -> f64 {
    (p.x - v0.x) * (v1.y - v0.y) - (p.y - v0.y) * (v1.x - v0.x)
}

#[inline(always)]
pub fn level_of_detail(screen_area: f64, uv: [UV; 3], size: f64) -> f64 {
    let area_uv = edge_function(uv[0], uv[1], uv[2]).abs();

    if screen_area < 1e-5 {
        return 0.0;
    }

    let rho = ((area_uv * size * size) / screen_area);
    let lod = 0.5 * rho.log2();

    lod
}

#[inline(always)]
pub fn point_inside_triangle(v0: Vector2, v1: Vector2, v2: Vector2, p: Vector2) -> bool {
    let ef0 = edge_function(v0, v1, p);
    let ef1 = edge_function(v1, v2, p);
    let ef2 = edge_function(v2, v0, p);

    ef0 <= 0.0 && ef1 <= 0.0 && ef2 <= 0.0
}
