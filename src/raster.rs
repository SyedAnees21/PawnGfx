use crate::{
    color::Color,
    draw::Face_NORMALS,
    geometry::{Triangles, bounding_rect, edge_function},
    math::{Matrix4, Vector2, Vector3, Vector4},
};

pub fn draw_call<F, D>(
    frame_buffer: &mut F,
    depth_buffer: &mut D,
    w: i32,
    h: i32,
    light: Vector3,
    mvp: Matrix4,
    triangles: Triangles,
) where
    F: AsMut<[u8]> + ?Sized,
    D: AsMut<[f64]> + ?Sized,
{
    let frame = frame_buffer.as_mut();
    let depth = depth_buffer.as_mut();

    for (idx, (v0, v1, v2)) in triangles.enumerate() {
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

        draw_triangle(frame, depth, w, h, light, Face_NORMALS[idx / 2], v0, v1, v2);
    }
}

pub fn draw_triangle(
    frame_buffer: &mut [u8],
    depth_buffer: &mut [f64],
    w: i32,
    h: i32,
    light: Vector3,
    face_normal: Vector3,
    (x0, y0, z0): (f64, f64, f64),
    (x1, y1, z1): (f64, f64, f64),
    (x2, y2, z2): (f64, f64, f64),
) {
    let frame = frame_buffer.as_mut();

    let v0 = Vector2::new(x0, y0);
    let v1 = Vector2::new(x1, y1);
    let v2 = Vector2::new(x2, y2);

    if is_backfacing(v0, v1, v2) {
        return;
    }

    let (min, max) = bounding_rect(v0, v1, v2);

    let min_x = min.x.max(0.0) as i32;
    let min_y = min.y.max(0.0) as i32;
    let max_x = max.x.min((w - 1) as f64) as i32;
    let max_y = max.y.min((h - 1) as f64) as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
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
                    let intensity = face_normal.normalize().dot(&light).max(0.0);
                    let color = Color::from_hex("#c19a6b").unwrap() * intensity;

                    depth_buffer[depth_index] = z;
                    frame[pixel_index..pixel_index + 4]
                        .copy_from_slice(color.to_rgba8().as_slice());
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
    let screen_x = (v_ndc.x + 1.0) * 0.5 * width;
    let screen_y = (1.0 - (v_ndc.y + 1.0) * 0.5) * height;

    (screen_x, screen_y, v_ndc.z)
}

pub fn is_backfacing(v0: Vector2, v1: Vector2, v2: Vector2) -> bool {
    edge_function(v0, v1, v2) < 0.0
}

#[allow(unused)]
pub fn outside_ndc_check(v: &Vector4) -> bool {
    v.x < -1.0 || v.x > 1.0 || v.y < -1.0 || v.y > 1.0 || v.z < -1.0 || v.z > 1.0
}

#[allow(unused)]
pub fn clip_volume_check(v_clip: &Vector4) -> bool {
    v_clip.x < -v_clip.w
        || v_clip.x > v_clip.w
        || v_clip.y < -v_clip.w
        || v_clip.y > v_clip.w
        || v_clip.z < -v_clip.w
        || v_clip.z > v_clip.w
}
