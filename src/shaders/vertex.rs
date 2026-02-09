use crate::{math::{Matrix4, Vector3}, shaders::Flat};



pub trait Vertex {
    type Uniforms;
    type Out;
    
    fn process_vertices(&self, rotation_matrix: Matrix4, vertices: &[Vector3]) -> Self::Out;
}


pub trait Fragment {
    type In;
    fn process_fragment(&self);
}
// impl VertexShader for Flat {
//     fn process_vertices(&self, vertices: &[Vector3]) -> Self::VertexOut {
        
//     }
// }