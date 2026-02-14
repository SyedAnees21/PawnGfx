use crate::math::{Vector3, Vector4};
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    pub data: [[f64; 4]; 4],
}

impl Matrix4 {
    pub const IDENTITY: Matrix4 = Matrix4 {
        data: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    #[inline(always)]
    pub fn identity() -> Matrix4 {
        Matrix4::IDENTITY
    }

    #[inline(always)]
    pub fn transpose(&self) -> Matrix4 {
        let mut transposed = Matrix4 {
            data: [[0.0; 4]; 4],
        };

        for i in 0..4 {
            for j in 0..4 {
                transposed.data[j][i] = self.data[i][j];
            }
        }

        transposed
    }

    #[inline(always)]
    pub fn inverse(&self) -> Self {
        let m = self.data;

        // Sub-determinants for Row-Major indexing: m[row][col]
        let s0 = m[0][0] * m[1][1] - m[0][1] * m[1][0];
        let s1 = m[0][0] * m[1][2] - m[0][2] * m[1][0];
        let s2 = m[0][0] * m[1][3] - m[0][3] * m[1][0];
        let s3 = m[0][1] * m[1][2] - m[0][2] * m[1][1];
        let s4 = m[0][1] * m[1][3] - m[0][3] * m[1][1];
        let s5 = m[0][2] * m[1][3] - m[0][3] * m[1][2];

        let c5 = m[2][2] * m[3][3] - m[2][3] * m[3][2];
        let c4 = m[2][1] * m[3][3] - m[2][3] * m[3][1];
        let c3 = m[2][1] * m[3][2] - m[2][2] * m[3][1];
        let c2 = m[2][0] * m[3][3] - m[2][3] * m[3][0];
        let c1 = m[2][0] * m[3][2] - m[2][2] * m[3][0];
        let c0 = m[2][0] * m[3][1] - m[2][1] * m[3][0];

        // Determinant calculation
        let det = s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0;

        if det.abs() < 1e-9 {
            return Matrix4::IDENTITY;
        }

        let inv_det = 1.0 / det;
        let mut inv = [[0.0; 4]; 4];

        // Row 0
        inv[0][0] = (m[1][1] * c5 - m[1][2] * c4 + m[1][3] * c3) * inv_det;
        inv[0][1] = (-m[0][1] * c5 + m[0][2] * c4 - m[0][3] * c3) * inv_det;
        inv[0][2] = (m[3][1] * s5 - m[3][2] * s4 + m[3][3] * s3) * inv_det;
        inv[0][3] = (-m[2][1] * s5 + m[2][2] * s4 - m[2][3] * s3) * inv_det;

        // Row 1
        inv[1][0] = (-m[1][0] * c5 + m[1][2] * c2 - m[1][3] * c1) * inv_det;
        inv[1][1] = (m[0][0] * c5 - m[0][2] * c2 + m[0][3] * c1) * inv_det;
        inv[1][2] = (-m[3][0] * s5 + m[3][2] * s2 - m[3][3] * s1) * inv_det;
        inv[1][3] = (m[2][0] * s5 - m[2][2] * s2 + m[2][3] * s1) * inv_det;

        // Row 2
        inv[2][0] = (m[1][0] * c4 - m[1][1] * c2 + m[1][3] * c0) * inv_det;
        inv[2][1] = (-m[0][0] * c4 + m[0][1] * c2 - m[0][3] * c0) * inv_det;
        inv[2][2] = (m[3][0] * s4 - m[3][1] * s2 + m[3][3] * s0) * inv_det;
        inv[2][3] = (-m[2][0] * s4 + m[2][1] * s2 - m[2][3] * s0) * inv_det;

        // Row 3
        inv[3][0] = (-m[1][0] * c3 + m[1][1] * c1 - m[1][2] * c0) * inv_det;
        inv[3][1] = (m[0][0] * c3 - m[0][1] * c1 + m[0][2] * c0) * inv_det;
        inv[3][2] = (-m[3][0] * s3 + m[3][1] * s1 - m[3][2] * s0) * inv_det;
        inv[3][3] = (m[2][0] * s3 - m[2][1] * s1 + m[2][2] * s0) * inv_det;

        Self { data: inv }
    }

    #[inline(always)]
    pub fn from_transforms(position: Vector3, scale: Vector3, rotation: Vector3) -> Self {
        let scale_m = Self::scale_matrix(scale.x, scale.y, scale.z);
        let rotation_m = Self::rotation_matrix(rotation);
        let translation_m = Self::translation_matrix(position.x, position.y, position.z);

        translation_m * rotation_m * scale_m
    }

    #[inline(always)]
    pub fn scale_matrix(sx: f64, sy: f64, sz: f64) -> Matrix4 {
        Matrix4 {
            data: [
                [sx, 0.0, 0.0, 0.0],
                [0.0, sy, 0.0, 0.0],
                [0.0, 0.0, sz, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline(always)]
    pub fn translation_matrix(tx: f64, ty: f64, tz: f64) -> Matrix4 {
        Matrix4 {
            data: [
                [1.0, 0.0, 0.0, tx],
                [0.0, 1.0, 0.0, ty],
                [0.0, 0.0, 1.0, tz],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline(always)]
    pub fn rotation_y(angle_rad: f64) -> Matrix4 {
        let c = angle_rad.cos();
        let s = angle_rad.sin();

        Matrix4 {
            data: [
                [c, 0.0, s, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-s, 0.0, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline(always)]
    pub fn rotation_x(angle_rad: f64) -> Matrix4 {
        let c = angle_rad.cos();
        let s = angle_rad.sin();

        Matrix4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, c, -s, 0.0],
                [0.0, s, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline(always)]
    pub fn rotation_z(angle_rad: f64) -> Matrix4 {
        let c = angle_rad.cos();
        let s = angle_rad.sin();

        Matrix4 {
            data: [
                [c, -s, 0.0, 0.0],
                [s, c, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline(always)]
    pub fn rotation_matrix(euler: Vector3) -> Matrix4 {
        let rx = Matrix4::rotation_x(euler.x.to_radians());
        let ry = Matrix4::rotation_y(euler.y.to_radians());
        let rz = Matrix4::rotation_z(euler.z.to_radians());

        rz * ry * rx
    }

    #[inline(always)]
    pub fn projection_matrix(fov_rad: f64, aspect: f64, near: f64, far: f64) -> Matrix4 {
        let f = 1.0 / (fov_rad / 2.0).tan();
        let nf = 1.0 / (near - far);

        Matrix4 {
            data: [
                [f / aspect, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (far + near) * nf, (2.0 * far * near) * nf],
                [0.0, 0.0, -1.0, 0.0],
            ],
        }
    }

    #[inline(always)]
    pub fn perspective_matrix(fov_rad: f64, aspect: f64, near: f64, far: f64) -> Matrix4 {
        Self::projection_matrix(fov_rad, aspect, near, far)
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;

    #[inline(always)]
    fn mul(self, rhs: Matrix4) -> Matrix4 {
        let mut result = Matrix4 {
            data: [[0.0; 4]; 4],
        };

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }

        result
    }
}

impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    #[inline(always)]
    fn mul(self, rhs: Vector4) -> Vector4 {
        let x = self.data[0][0] * rhs.x
            + self.data[0][1] * rhs.y
            + self.data[0][2] * rhs.z
            + self.data[0][3] * rhs.w;
        let y = self.data[1][0] * rhs.x
            + self.data[1][1] * rhs.y
            + self.data[1][2] * rhs.z
            + self.data[1][3] * rhs.w;
        let z = self.data[2][0] * rhs.x
            + self.data[2][1] * rhs.y
            + self.data[2][2] * rhs.z
            + self.data[2][3] * rhs.w;
        let w = self.data[3][0] * rhs.x
            + self.data[3][1] * rhs.y
            + self.data[3][2] * rhs.z
            + self.data[3][3] * rhs.w;

        Vector4 { x, y, z, w }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix3 {
    pub data: [[f64; 3]; 3],
}

impl Matrix3 {
    pub const IDENTITY: Matrix3 = Matrix3 {
        data: [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ],
    };

    #[inline(always)]
    pub fn identity() -> Matrix3 {
        Matrix3::IDENTITY
    }

    /// Creates a TBN matrix (Tangent Space -> World/Model Space)
    /// The vectors t, b, and n become the COLUMNS of the matrix.
    #[inline(always)]
    pub fn from_tbn(t: Vector3, b: Vector3, n: Vector3) -> Matrix3 {
        Matrix3 {
            data: [
                [t.x, b.x, n.x],
                [t.y, b.y, n.y],
                [t.z, b.z, n.z],
            ],
        }
    }

    #[inline(always)]
    pub fn transpose(&self) -> Matrix3 {
        let mut transposed = Matrix3 {
            data: [[0.0; 3]; 3],
        };
        for i in 0..3 {
            for j in 0..3 {
                transposed.data[j][i] = self.data[i][j];
            }
        }
        transposed
    }

    #[inline(always)]
    pub fn determinant(&self) -> f64 {
        let m = self.data;
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) -
        m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0]) +
        m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    }
}

impl Mul for Matrix3 {
    type Output = Matrix3;

    #[inline(always)]
    fn mul(self, rhs: Matrix3) -> Matrix3 {
        let mut result = Matrix3 {
            data: [[0.0; 3]; 3],
        };
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        result
    }
}

impl Mul<Vector3> for Matrix3 {
    type Output = Vector3;

    #[inline(always)]
    fn mul(self, rhs: Vector3) -> Vector3 {
        let x = self.data[0][0] * rhs.x + self.data[0][1] * rhs.y + self.data[0][2] * rhs.z;
        let y = self.data[1][0] * rhs.x + self.data[1][1] * rhs.y + self.data[1][2] * rhs.z;
        let z = self.data[2][0] * rhs.x + self.data[2][1] * rhs.y + self.data[2][2] * rhs.z;
        Vector3 { x, y, z }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AffineMatrices {
    pub model: Matrix4,
    pub view: Matrix4,
    pub projection: Matrix4,
    pub mvp: Matrix4,
    pub normal: Matrix4,
}

impl AffineMatrices {
    #[inline(always)]
    pub fn from_mvp(model: Matrix4, view: Matrix4, projection: Matrix4) -> Self {
        let mvp = projection * view * model;
        let normal = model.inverse().transpose();

        Self {
            model,
            view,
            projection,
            mvp,
            normal,
        }
    }
}
