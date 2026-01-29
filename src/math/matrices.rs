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

    pub fn identity() -> Matrix4 {
        Matrix4::IDENTITY
    }

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

    pub fn rotation_matrix(euler: Vector3) -> Matrix4 {
        let rx = Matrix4::rotation_x(euler.x.to_radians());
        let ry = Matrix4::rotation_y(euler.y.to_radians());
        let rz = Matrix4::rotation_z(euler.z.to_radians());

        rz * ry * rx
    }

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

    pub fn perspective_matrix(fov_rad: f64, aspect: f64, near: f64, far: f64) -> Matrix4 {
        Self::projection_matrix(fov_rad, aspect, near, far)
    }

    // pub fn view_matrix(eye: [f64; 3], center: [f64; 3], up: [f64; 3]) -> Matrix4 {
    //     let f = normalize([
    //         center[0] - eye[0],
    //         center[1] - eye[1],
    //         center[2] - eye[2],
    //     ]);
    //     let s = normalize(cross(f, up));
    //     let u = cross(s, f);

    //     Matrix4 {
    //         data: [
    //             [s[0], u[0], -f[0], 0.0],
    //             [s[1], u[1], -f[1], 0.0],
    //             [s[2], u[2], -f[2], 0.0],
    //             [
    //                 -dot(s, eye),
    //                 -dot(u, eye),
    //                 dot(f, eye),
    //                 1.0,
    //             ],
    //         ],
    //     }
    // }
}

impl Mul for Matrix4 {
    type Output = Matrix4;

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
