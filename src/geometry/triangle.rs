use crate::{
    geometry::{Normal, UV, VertexAttributes, edge_function, mesh::Mesh},
    math::{Vector2, Vector3},
};

pub struct Triangles<'a> {
    pub mesh: &'a Mesh,
    pub counter: usize,
}

impl Iterator for Triangles<'_> {
    type Item = [VertexAttributes; 3];

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter + 2 >= self.mesh.indices.len() {
            return None;
        }

        let mut v_attributes = [VertexAttributes::default(); 3];

        for i in 0..3 {
            let index = self.counter + i;

            let v_id = self.mesh.indices.v[index];
            let n_id = self.mesh.indices.n[index];
            let uv_id = self.mesh.indices.uv[index];

            let v = self.mesh.vertices[v_id];
            let n = self.mesh.normals[n_id];
            let uv = self.mesh.uv[uv_id];

            let tangent = self.mesh.tangents[v_id];
            let bi_tangent = self.mesh.bi_tangents[v_id];

            v_attributes[i].set_position(v);
            v_attributes[i].set_normal(n);
            v_attributes[i].set_uv(uv);
            v_attributes[i].set_tangent(tangent);
            v_attributes[i].set_bi_tangent(bi_tangent);
        }

        self.counter += 3;
        Some(v_attributes)
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
