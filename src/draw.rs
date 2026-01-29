use crate::math::{Matrix4, Vector3, Vector4};

pub fn draw_line<T>(mut frame: T, w: i32, x0: i32, y0: i32, x1: i32, y1: i32)
where
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

    loop {
        if x >= 0 && y >= 0 && x < w && y < w {
            let idx = ((y * w + x) * 4) as usize;
            frame[idx] = 255;
            frame[idx + 1] = 255;
            frame[idx + 2] = 255;
            frame[idx + 3] = 255;
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
    }
}

const CUBE_VERTS: [Vector3; 8] = [
    Vector3::new(-1.0, -1.0, -1.0),
    Vector3::new(1.0, -1.0, -1.0),
    Vector3::new(1.0, 1.0, -1.0),
    Vector3::new(-1.0, 1.0, -1.0),
    Vector3::new(-1.0, -1.0, 1.0),
    Vector3::new(1.0, -1.0, 1.0),
    Vector3::new(1.0, 1.0, 1.0),
    Vector3::new(-1.0, 1.0, 1.0),
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

pub fn draw_cube<T>(mut frame: T, mvp: Matrix4, width: f64, height: f64)
where
    T: AsMut<[u8]>,
{
    let frame = frame.as_mut();

    let projected: Vec<(i32, i32)> = CUBE_VERTS
        .iter()
        .map(|v| {
            let v4: Vector4 = (*v, 1.0).into();
            let v_clip = mvp * v4;

            let ndc_x = v_clip.x / v_clip.w;
            let ndc_y = v_clip.y / v_clip.w;

            let screen_x = ((ndc_x + 1.0) * 0.5 * width) as i32;
            let screen_y = ((1.0 - (ndc_y + 1.0) * 0.5) * height) as i32;

            (screen_x, screen_y)
        })
        .collect();

    for &(start, end) in &EDGES {
        let (x0, y0) = projected[start];
        let (x1, y1) = projected[end];
        draw_line(&mut *frame, width as i32, x0, y0, x1, y1);
    }
}
