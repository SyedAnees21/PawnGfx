use crate::{geometry::triangle::Triangles, math::Vector3};

pub type Index = usize;
pub type Indices = Vec<Index>;
pub type Vertices = Vec<Vector3>;

pub struct Mesh {
    pub vertices: Vertices,
    pub indices: Indices,
}

impl Mesh {
    pub fn new(vertices: Vertices, indices: Indices) -> Self {
        Self { vertices, indices }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn index_count(&self) -> usize {
        self.indices.len()
    }

    pub fn triangles(&self) -> Triangles {
        Triangles::new(self)
    }
}
