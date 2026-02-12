use std::ops::Index;

use crate::{
    geometry::{Idx, NIdx, Normal, Normals, TIdx, UV, VIdx, Vertex, Vertices, triangle::Triangles},
    math::{Vector2, Vector3},
};

#[derive(Debug)]
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

#[derive(Debug)]
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
    pub fn new(
        vertices: Vertices,
        mut uv: Vec<UV>,
        mut indices: Indices,
        mut vnormals: Normals,
    ) -> Self {
        Self::bake_mesh(&vertices, &mut indices, &mut uv, &mut vnormals);

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

    fn bake_mesh(
        vertices: &Vertices,
        indices: &mut Indices,
        uv: &mut Vec<UV>,
        normals: &mut Normals,
    ) {
        if normals.is_empty() {
            Self::bake_normals(vertices, indices, normals);
        }
    }

    fn bake_normals(vertices: &Vertices, indices: &mut Indices, normals: &mut Normals) {
        normals.resize(vertices.len(), Normal::default());
        let count = indices.v.len() / 3;

        for i in 0..count {
            let id0 = indices.v[i * 3];
            let id1 = indices.v[i * 3 + 1];
            let id2 = indices.v[i * 3 + 2];

            let v0 = vertices[id0];
            let v1 = vertices[id1];
            let v2 = vertices[id2];

            let f_normal = (v1 - v0).cross(&(v2 - v0));

            indices.n[i * 3] = id0;
            indices.n[i * 3 + 1] = id1;
            indices.n[i * 3 + 2] = id2;

            normals[id0] = normals[id0] + f_normal;
            normals[id1] = normals[id1] + f_normal;
            normals[id2] = normals[id2] + f_normal;
        }

        normals.iter_mut().for_each(|n| *n = n.normalize());
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
