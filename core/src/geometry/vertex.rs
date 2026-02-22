use crate::geometry::{BiTangent, Normal, Tangent, UV, Vector3};

#[derive(Default, Debug, Clone, Copy)]
pub struct VertexAttributes {
    pub position: Vector3,
    pub normal: Normal,
    pub uv: UV,
    pub tangent: Tangent,
    pub bi_tangent: BiTangent,
}

impl VertexAttributes {
    #[inline(always)]
    pub fn set_position(&mut self, v: Vector3) {
        self.position = v;
    }

    #[inline(always)]
    pub fn set_normal(&mut self, v: Normal) {
        self.normal = v;
    }

    #[inline(always)]
    pub fn set_uv(&mut self, v: UV) {
        self.uv = v;
    }

    #[inline(always)]
    pub fn set_tangent(&mut self, v: Tangent) {
        self.tangent = v;
    }

    #[inline(always)]
    pub fn set_bi_tangent(&mut self, v: BiTangent) {
        self.bi_tangent = v;
    }
}
