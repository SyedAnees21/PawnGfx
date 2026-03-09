use pcore::geometry::Normal;


#[derive(Clone, Copy, Default)]
pub struct PackedNormal(pub u32);

impl PackedNormal {
    /// Packs a unit vector into 16-bit X and 16-bit Y.
    /// Range: [-1.0, 1.0] -> [0, 65535]
    pub fn pack(n: Normal) -> Self {
        let ux = (((n.x * 0.5 + 0.5) * 65535.0) as u32).clamp(0, 65535);
        let uy = (((n.y * 0.5 + 0.5) * 65535.0) as u32).clamp(0, 65535);
        Self(ux | (uy << 16))
    }
}