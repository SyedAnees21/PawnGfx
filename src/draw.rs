use crate::{
    geometry::{Triangles, bounding_rect, edge_function, point_inside_triangle},
    math::{Matrix4, Vector2, Vector3, Vector4},
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

// pub const CUBE_VERTS: [Vector3; 8] = [
//     Vector3::new(-1.0, -1.0, -1.0),
//     Vector3::new(1.0, -1.0, -1.0),
//     Vector3::new(1.0, 1.0, -1.0),
//     Vector3::new(-1.0, 1.0, -1.0),
//     Vector3::new(-1.0, -1.0, 1.0),
//     Vector3::new(1.0, -1.0, 1.0),
//     Vector3::new(1.0, 1.0, 1.0),
//     Vector3::new(-1.0, 1.0, 1.0),
// ];

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

// pub const CUBE_TRIS: [usize; 36] = [
//     0, 1, 2,
//     2, 3, 0,
//     4, 6, 5,
//     6, 4, 7,
//     0, 3, 7,
//     7, 4, 0,
//     1, 5, 6,
//     6, 2, 1,
//     3, 2, 6,
//     6, 7, 3,
//     0, 4, 5,
//     5, 1, 0
// ];

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

pub fn draw_call<F, D>(
    frame_buffer: &mut F,
    depth_buffer: &mut D,
    w: i32,
    h: i32,
    mvp: Matrix4,
    triangles: Triangles,
) where
    F: AsMut<[u8]> + ?Sized,
    D: AsMut<[f64]>,
{
    let frame = frame_buffer.as_mut();
    let depth = depth_buffer.as_mut();

    for (v0, v1, v2) in triangles {
        let v0_clip = transform_to_clip_space(&v0, &mvp);
        let v1_clip = transform_to_clip_space(&v1, &mvp);
        let v2_clip = transform_to_clip_space(&v2, &mvp);

        if v0_clip.w <= 0.0 || v1_clip.w <= 0.0 || v2_clip.w <= 0.0 {
            continue;
        }

        let v0_ndc = v0_clip / v0_clip.w;
        let v1_ndc = v1_clip / v1_clip.w;
        let v2_ndc = v2_clip / v2_clip.w;

        let v0 = clip_to_screen(&v0_ndc, w as f64, h as f64);
        let v1 = clip_to_screen(&v1_ndc, w as f64, h as f64);
        let v2 = clip_to_screen(&v2_ndc, w as f64, h as f64);

        draw_triangle(frame, depth, w, h, v0, v1, v2);
    }
}

pub fn draw_triangle(
    frame_buffer: &mut [u8],
    depth_buffer: &mut [f64],
    w: i32,
    h: i32,
    (x0, y0, z0): (f64, f64, f64),
    (x1, y1, z1): (f64, f64, f64),
    (x2, y2, z2): (f64, f64, f64),
) {
    let frame = frame_buffer.as_mut();

    let v0 = Vector2::new(x0, y0);
    let v1 = Vector2::new(x1, y1);
    let v2 = Vector2::new(x2, y2);

    let (min, max) = bounding_rect(v0, v1, v2);

    let min_x = min.x.max(0.0) as i32;
    let min_y = min.y.max(0.0) as i32;
    let max_x = max.x.min((w - 1) as f64) as i32;
    let max_y = max.y.min((h - 1) as f64) as i32;

    for y in min_y..=max_y {
        for x in min_x..max_x {
            let p = Vector2::new(x as f64 + 0.5, y as f64 + 0.5);

            {
                let area = edge_function(v0, v1, v2);

                let w0 = edge_function(v1, v2, p) / area;
                let w1 = edge_function(v2, v0, p) / area;
                let w2 = edge_function(v0, v1, p) / area;

                if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                    continue;
                }

                let z = w0 * z0 + w1 * z1 + w2 * z2;
                let depth_index = (y * w + x) as usize;
                let pixel_index = (depth_index * 4) as usize;

                if z < depth_buffer[depth_index] {
                    depth_buffer[depth_index] = z;
                    frame[pixel_index] = 255;
                    frame[pixel_index + 1] = 255;
                    frame[pixel_index + 2] = 255;
                    frame[pixel_index + 3] = 255;
                }
            }
        }
    }
}

pub fn transform_to_clip_space(v: &Vector3, mvp: &Matrix4) -> Vector4 {
    let v4 = Vector4::from((*v, 1.0));
    *mvp * v4
}

pub fn clip_to_screen(v_ndc: &Vector4, width: f64, height: f64) -> (f64, f64, f64) {
    let screen_x = ((v_ndc.x + 1.0) * 0.5 * width);
    let screen_y = ((1.0 - (v_ndc.y + 1.0) * 0.5) * height);

    (screen_x, screen_y, v_ndc.z)
}

pub fn outside_ndc_check(v: &Vector4) -> bool {
    v.x < -1.0 || v.x > 1.0 || v.y < -1.0 || v.y > 1.0 || v.z < -1.0 || v.z > 1.0
}

pub fn clip_volume_check(v_clip: &Vector4) -> bool {
    v_clip.x < -v_clip.w
        || v_clip.x > v_clip.w
        || v_clip.y < -v_clip.w
        || v_clip.y > v_clip.w
        || v_clip.z < -v_clip.w
        || v_clip.z > v_clip.w
}
