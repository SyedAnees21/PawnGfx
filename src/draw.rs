use crate::{
    math::{Matrix4, Vector3},
    raster::{clip_to_screen, clip_volume_check, transform_to_clip_space},
};

pub fn draw_line<T>(
    mut frame: T,
    depth_buffer: &mut [f64],
    w: i32,
    h: i32,
    x0: i32,
    y0: i32,
    z0: f64,
    x1: i32,
    y1: i32,
    z1: f64,
) where
    T: AsMut<[u8]>,
{
    let frame = frame.as_mut();
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut x = x0;
    let mut y = y0;

    let length = ((x1 - x0).abs().max((y1 - y0).abs())) as f64;
    let mut step = 0.0;

    loop {
        if x >= 0 && y >= 0 && x < w && y < h {
            let t = if length > 0.0 { step / length } else { 0.0 };
            let z = z0 * (1.0 - t) + z1 * t;
            let depth_index = (y * w + x) as usize;

            if z < depth_buffer[depth_index] {
                depth_buffer[depth_index] = z;

                let pixel_index = (depth_index * 4) as usize;
                frame[pixel_index] = 255;
                frame[pixel_index + 1] = 255;
                frame[pixel_index + 2] = 255;
                frame[pixel_index + 3] = 255;
            }
        }
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }

        step += 1.0;
    }
}

pub const CUBE_VERTS: [Vector3; 8] = [
    Vector3::new(-1.0, -1.0, -1.0), // 0
    Vector3::new(1.0, -1.0, -1.0),  // 1
    Vector3::new(1.0, 1.0, -1.0),   // 2
    Vector3::new(-1.0, 1.0, -1.0),  // 3
    Vector3::new(-1.0, -1.0, 1.0),  // 4
    Vector3::new(1.0, -1.0, 1.0),   // 5
    Vector3::new(1.0, 1.0, 1.0),    // 6
    Vector3::new(-1.0, 1.0, 1.0),   // 7
];

const EDGES: [(usize, usize); 12] = [
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 0),
    (4, 5),
    (5, 6),
    (6, 7),
    (7, 4),
    (0, 4),
    (1, 5),
    (2, 6),
    (3, 7),
];

pub const CUBE_TRIS: [usize; 36] = [
    // FRONT  (-Z)
    0, 2, 1, 0, 3, 2, // BACK   (+Z)
    4, 5, 6, 4, 6, 7, // LEFT   (-X)
    0, 7, 3, 0, 4, 7, // RIGHT  (+X)
    1, 2, 6, 1, 6, 5, // TOP    (+Y)
    3, 7, 6, 3, 6, 2, // BOTTOM (-Y)
    0, 1, 5, 0, 5, 4,
];

pub fn draw_cube<T>(mut frame: T, mvp: Matrix4, width: f64, height: f64)
where
    T: AsMut<[u8]>,
{
    let frame = frame.as_mut();
    let mut depth_buffer = vec![f64::INFINITY; (width * height) as usize];

    for edge in &EDGES {
        let (start, end) = *edge;

        let v0_clip = transform_to_clip_space(&CUBE_VERTS[start], &mvp);
        let v1_clip = transform_to_clip_space(&CUBE_VERTS[end], &mvp);

        if v0_clip.w <= 0.0
            && v1_clip.w <= 0.0
            && clip_volume_check(&v0_clip)
            && clip_volume_check(&v1_clip)
        {
            continue;
        }

        let v0_ndc = v0_clip / v0_clip.w;
        let v1_ndc = v1_clip / v1_clip.w;

        let (x0, y0, z0) = clip_to_screen(&v0_ndc, width, height);
        let (x1, y1, z1) = clip_to_screen(&v1_ndc, width, height);

        draw_line(
            &mut *frame,
            &mut depth_buffer,
            width as i32,
            height as i32,
            x0 as i32,
            y0 as i32,
            z0,
            x1 as i32,
            y1 as i32,
            z1,
        );
    }
}
