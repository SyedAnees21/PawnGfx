use std::ops::Index;

use crate::{
    geometry::{Idx, NIdx, Normal, Normals, TIdx, UV, VIdx, Vertex, Vertices, triangle::Triangles},
    math::{Vector2, Vector3},
};

pub struct Indices {
    pub v: Vec<Idx>,
    pub n: Vec<Idx>,
    pub t: Vec<Idx>,
}

impl Default for Indices {
    fn default() -> Self {
        Self {
            v: vec![],
            n: vec![],
            t: vec![],
        }
    }
}

impl Indices {
    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn index(&self, index: Idx) -> (VIdx, NIdx, TIdx) {
        if index >= self.len() {
            return (0, 0, 0);
        }
        (self.v[index], self.n[index], self.t[index])
    }

    pub fn push_v_index(&mut self, idx: Idx) {
        self.v.push(idx);
    }

    pub fn push_n_index(&mut self, idx: Idx) {
        self.n.push(idx);
    }

    pub fn push_uv_index(&mut self, idx: Idx) {
        self.t.push(idx);
    }
}

pub struct Mesh {
    pub vertices: Vertices,
    pub normals: Normals,
    pub uv: Vec<Vector2>,
    pub indices: Indices,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            vertices: vec![],
            normals: vec![],
            uv: vec![],
            indices: Indices::default(),
        }
    }
}

impl Mesh {
    pub fn new(vertices: Vertices, uv: Vec<Vector2>, indices: Indices, vnormals: Normals) -> Self {
        Self {
            vertices,
            indices,
            uv,
            normals: vnormals,
        }
    }

    pub fn from_vertices_faces(vertices: Vertices, faces: Vec<Idx>) -> Self {
        Self {
            vertices,
            indices: Indices {
                v: faces,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn index_count(&self) -> usize {
        self.indices.len()
    }

    pub fn get_indices(&self, index: Idx) -> (VIdx, NIdx, TIdx) {
        self.indices.index(index)
    }

    pub fn iter_triangles(&self) -> Triangles {
        Triangles::new(self)
    }

    pub fn has_normals(&self) -> bool {
        !self.normals.is_empty()
    }

    pub fn has_uv(&self) -> bool {
        !self.uv.is_empty()
    }
}

pub struct IterVertices<'a> {
    mesh: &'a Mesh,
    counter: usize,
}

impl Iterator for IterVertices<'_> {
    type Item = (Vertex, Normal, UV);

    fn next(&mut self) -> Option<Self::Item> {
        if self.mesh.indices.len() <= self.counter {
            return None;
        }

        let (vi, ni, uvi) = self.mesh.get_indices(self.counter);
        let v = self.mesh.vertices[vi];
        let n = self.mesh.normals[ni];
        let uv = self.mesh.uv[uvi];

        Some((v, n, uv))
    }
}

pub struct IterNormals<'a> {
    mesh: &'a Mesh,
    counter: usize,
}

impl Iterator for IterNormals<'_> {
    type Item = Normal;

    fn next(&mut self) -> Option<Self::Item> {
        if self.mesh.indices.n.len() == 0 || self.mesh.indices.n.len() <= self.counter {
            return None;
        }

        let idx = self.counter;
        self.counter += 1;

        Some(self.mesh.normals[idx])
    }
}

pub struct IterUV<'a> {
    mesh: &'a Mesh,
    counter: usize,
}

impl Iterator for IterUV<'_> {
    type Item = UV;

    fn next(&mut self) -> Option<Self::Item> {
        if self.mesh.indices.t.len() == 0 || self.mesh.indices.t.len() <= self.counter {
            return None;
        }

        let idx = self.counter;
        self.counter += 1;

        Some(self.mesh.uv[idx])
    }
}
