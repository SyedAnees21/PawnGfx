use crate::{
    geometry::{Triangles, bounding_rect, edge_function},
    math::{self, Matrix4, Vector2, Vector3, Vector4},
    scene::Texture,
    shaders::{FragmentShader, GlobalUniforms, Varyings, VertexIn, VertexShader},
};

pub fn draw_call<F, D>(
    frame_buffer: &mut F,
    depth_buffer: &mut D,
    global_uniforms: GlobalUniforms,
    light: Vector3,
    texture: &Texture,
    triangles: Triangles,
) where
    F: AsMut<[u8]> + ?Sized,
    D: AsMut<[f64]> + ?Sized,
{
    let frame = frame_buffer.as_mut();
    let depth = depth_buffer.as_mut();

    let w = global_uniforms.screen_width as i32;
    let h = global_uniforms.screen_height as i32;

    for (_idx, (v, n, uv)) in triangles.enumerate() {
        let [v0, v1, v2] = v;

        let v0_clip = transform_to_clip_space(v0, global_uniforms.uniforms.mvp);
        let v1_clip = transform_to_clip_space(v1, global_uniforms.uniforms.mvp);
        let v2_clip = transform_to_clip_space(v2, global_uniforms.uniforms.mvp);

        if v0_clip.w <= 0.0 || v1_clip.w <= 0.0 || v2_clip.w <= 0.0 {
            continue;
        }

        let [n1, _, _] = n.unwrap();

        let face_normal = (global_uniforms.uniforms.normal * Vector4::from((n1, 0.0))).xyz();

        let inv_w0 = 1.0 / v0_clip.w;
        let inv_w1 = 1.0 / v1_clip.w;
        let inv_w2 = 1.0 / v2_clip.w;

        let mut v0_ndc = v0_clip * inv_w0;
        let mut v1_ndc = v1_clip * inv_w1;
        let mut v2_ndc = v2_clip * inv_w2;

        v0_ndc.w = inv_w0;
        v1_ndc.w = inv_w1;
        v2_ndc.w = inv_w2;

        let v0 = clip_to_screen(&v0_ndc, w as f64, h as f64);
        let v1 = clip_to_screen(&v1_ndc, w as f64, h as f64);
        let v2 = clip_to_screen(&v2_ndc, w as f64, h as f64);

        draw_triangle(
            frame,
            depth,
            w,
            h,
            light,
            face_normal,
            &texture,
            uv,
            v0,
            v1,
            v2,
        );
    }
}

pub fn draw_call_generic<VS, FS>(
    frame_buffer: &mut [u8],
    depth_buffer: &mut [f64],
    global_uniforms: &GlobalUniforms,
    texture: &Texture,
    triangles: Triangles,
    v_shader: &VS,
    f_shader: &FS,
) where
    VS: VertexShader,
    FS: FragmentShader,
{
    let w = global_uniforms.screen_width as i32;
    let h = global_uniforms.screen_height as i32;

    for (v, n, uv) in triangles {
        let [v0, v1, v2] = v;

        
        let face_normal = (v1 - v0).cross(&(v2 - v0)).normalize();
        let [n0, n1, n2] = n.unwrap_or([face_normal; 3]);
        let [uv0, uv1, uv2] = uv.unwrap_or([Vector2::ZERO; 3]);

        // v.into_iter().enumerate().map(|(ix, vert)| {
        //     v_shader.shade(VertexIn { position: vert, normal: n.unwrap_or(face_normal(v0, v1, v2))[ix], uv: (), face_normal: () }, u)
        // })
        
        let out0 = v_shader.shade(
            VertexIn {
                position: v0,
                normal: n0,
                uv: uv0,
                face_normal,
            },
            global_uniforms,
        );
        let out1 = v_shader.shade(
            VertexIn {
                position: v1,
                normal: n1,
                uv: uv1,
                face_normal,
            },
            global_uniforms,
        );
        let out2 = v_shader.shade(
            VertexIn {
                position: v2,
                normal: n2,
                uv: uv2,
                face_normal,
            },
            global_uniforms,
        );

        let v0_clip = out0.clip;
        let v1_clip = out1.clip;
        let v2_clip = out2.clip;

        if v0_clip.w <= 0.0 || v1_clip.w <= 0.0 || v2_clip.w <= 0.0 {
            continue;
        }

        let inv_w0 = 1.0 / v0_clip.w;
        let inv_w1 = 1.0 / v1_clip.w;
        let inv_w2 = 1.0 / v2_clip.w;

        let mut v0_ndc = v0_clip * inv_w0;
        let mut v1_ndc = v1_clip * inv_w1;
        let mut v2_ndc = v2_clip * inv_w2;

        v0_ndc.w = inv_w0;
        v1_ndc.w = inv_w1;
        v2_ndc.w = inv_w2;

        let v0_screen = clip_to_screen(&v0_ndc, w as f64, h as f64);
        let v1_screen = clip_to_screen(&v1_ndc, w as f64, h as f64);
        let v2_screen = clip_to_screen(&v2_ndc, w as f64, h as f64);

        if is_backfacing(v0_screen.0, v1_screen.0, v2_screen.0) {
            continue;
        }

        draw_triangle_shaded(
            frame_buffer,
            depth_buffer,
            w,
            h,
            global_uniforms,
            texture,
            f_shader,
            out0.vary,
            out1.vary,
            out2.vary,
            v0_screen,
            v1_screen,
            v2_screen,
        );
    }
}

fn draw_triangle_shaded<FS>(
    frame_buffer: &mut [u8],
    depth_buffer: &mut [f64],
    w: i32,
    h: i32,
    global_uniforms: &GlobalUniforms,
    texture: &Texture,
    f_shader: &FS,
    v0: Varyings,
    v1: Varyings,
    v2: Varyings,
    (s0, z0, inv_w0): (Vector2, f64, f64),
    (s1, z1, inv_w1): (Vector2, f64, f64),
    (s2, z2, inv_w2): (Vector2, f64, f64),
) where
    FS: FragmentShader,
{
    let area = edge_function(s0, s1, s2);
    let inv_area = 1.0 / area;

    let (min, max) = bounding_rect(s0, s1, s2);

    let min_x = min.x.max(0.0) as i32;
    let min_y = min.y.max(0.0) as i32;
    let max_x = max.x.min((w - 1) as f64) as i32;
    let max_y = max.y.min((h - 1) as f64) as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Vector2::new(x as f64 + 0.5, y as f64 + 0.5);

            let w0 = edge_function(s1, s2, p) * inv_area;
            let w1 = edge_function(s2, s0, p) * inv_area;
            let w2 = edge_function(s0, s1, p) * inv_area;

            if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                continue;
            }

            let bary = (w0, w1, w2);

            let inv_depth = math::barycentric_interpolate(w0, w1, w2, inv_w0, inv_w1, inv_w2);
            let z = math::perspective_interpolate(bary, inv_depth, (z0, z1, z2));

            let depth_index = (y * w + x) as usize;
            let pixel_index = depth_index * 4;

            if z < depth_buffer[depth_index] {
                let uv0 = v0.uv * inv_w0;
                let uv1 = v1.uv * inv_w1;
                let uv2 = v2.uv * inv_w2;

                let n0 = v0.normal * inv_w0;
                let n1 = v1.normal * inv_w1;
                let n2 = v2.normal * inv_w2;

                let p0 = v0.world_pos * inv_w0;
                let p1 = v1.world_pos * inv_w1;
                let p2 = v2.world_pos * inv_w2;

                let i0 = v0.intensity * inv_w0;
                let i1 = v1.intensity * inv_w1;
                let i2 = v2.intensity * inv_w2;

                let uv = math::perspective_interpolate(bary, inv_depth, (uv0, uv1, uv2));
                let normal = math::perspective_interpolate(bary, inv_depth, (n0, n1, n2));
                let world_pos = math::perspective_interpolate(bary, inv_depth, (p0, p1, p2));
                let intensity = math::perspective_interpolate(bary, inv_depth, (i0, i1, i2));

                let color = f_shader.shade(
                    Varyings {
                        uv,
                        normal,
                        world_pos,
                        intensity,
                    },
                    global_uniforms,
                    texture,
                );

                depth_buffer[depth_index] = z;
                frame_buffer[pixel_index..pixel_index + 4].copy_from_slice(&color.to_rgba8());
            }
        }
    }
}

pub fn draw_triangle(
    frame_buffer: &mut [u8],
    depth_buffer: &mut [f64],
    w: i32,
    h: i32,
    light: Vector3,
    face_normal: Vector3,
    texture: &Texture,
    uv: Option<[Vector2; 3]>,
    (v0, z0, inv_w0): (Vector2, f64, f64),
    (v1, z1, inv_w1): (Vector2, f64, f64),
    (v2, z2, inv_w2): (Vector2, f64, f64),
) {
    let frame = frame_buffer.as_mut();

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
                let inv_area = 1.0 / area;

                let w0 = edge_function(v1, v2, p) * inv_area;
                let w1 = edge_function(v2, v0, p) * inv_area;
                let w2 = edge_function(v0, v1, p) * inv_area;

                if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                    continue;
                }

                let bary_cords = (w0, w1, w2);
                let inv_depth_cords = (inv_w0, inv_w1, inv_w2);

                let z = persp_correct_interpolate(bary_cords, inv_depth_cords, (z0, z1, z2));

                let depth_index = (y * w + x) as usize;
                let pixel_index = (depth_index * 4) as usize;

                if z < depth_buffer[depth_index] {
                    let [uv0, uv1, uv2] = uv.unwrap();
                    let (uv0, uv1, uv2) = (uv0 * inv_w0, uv1 * inv_w1, uv2 * inv_w2);

                    let u = persp_correct_interpolate(
                        bary_cords,
                        inv_depth_cords,
                        (uv0.x, uv1.x, uv2.x),
                    );
                    let v = persp_correct_interpolate(
                        bary_cords,
                        inv_depth_cords,
                        (uv0.y, uv1.y, uv2.y),
                    );

                    let s_color = texture.bi_sample(u, v);

                    let intensity = face_normal.normalize().dot(&light).max(0.0);
                    let color = s_color * intensity;

                    depth_buffer[depth_index] = z;
                    frame[pixel_index..pixel_index + 4].copy_from_slice(&color.to_rgba8());
                }
            }
        }
    }
}

pub fn transform_to_clip_space(v: Vector3, mvp: Matrix4) -> Vector4 {
    let v4 = Vector4::from((v, 1.0));
    mvp * v4
}

pub fn clip_to_screen(v_ndc: &Vector4, width: f64, height: f64) -> (Vector2, f64, f64) {
    let screen_x = (v_ndc.x + 1.0) * 0.5 * width;
    let screen_y = (1.0 - (v_ndc.y + 1.0) * 0.5) * height;

    (Vector2::new(screen_x, screen_y), v_ndc.z, v_ndc.w)
}

pub fn is_backfacing(v0: Vector2, v1: Vector2, v2: Vector2) -> bool {
    edge_function(v0, v1, v2) < 0.0
}

pub fn interpolate(w0: f64, w1: f64, w2: f64, v0: f64, v1: f64, v2: f64) -> f64 {
    w0 * v0 + w1 * v1 + w2 * v2
}

pub fn persp_correct_interpolate(
    bary: (f64, f64, f64),
    inv_depth: (f64, f64, f64),
    elements: (f64, f64, f64),
) -> f64 {
    let (w0, w1, w2) = bary;
    let (inv_w0, inv_w1, inv_w2) = inv_depth;
    let (v0, v1, v2) = elements;

    let v_prime = interpolate(w0, w1, w2, v0, v1, v2);
    let inv_d_lerped = interpolate(w0, w1, w2, inv_w0, inv_w1, inv_w2);

    v_prime / inv_d_lerped
}

fn persp_interp_vec2(
    bary: (f64, f64, f64),
    inv_depth: (f64, f64, f64),
    v0: Vector2,
    v1: Vector2,
    v2: Vector2,
) -> Vector2 {
    Vector2::new(
        persp_correct_interpolate(bary, inv_depth, (v0.x, v1.x, v2.x)),
        persp_correct_interpolate(bary, inv_depth, (v0.y, v1.y, v2.y)),
    )
}

fn persp_interp_vec3(
    bary: (f64, f64, f64),
    inv_depth: (f64, f64, f64),
    v0: Vector3,
    v1: Vector3,
    v2: Vector3,
) -> Vector3 {
    Vector3::new(
        persp_correct_interpolate(bary, inv_depth, (v0.x, v1.x, v2.x)),
        persp_correct_interpolate(bary, inv_depth, (v0.y, v1.y, v2.y)),
        persp_correct_interpolate(bary, inv_depth, (v0.z, v1.z, v2.z)),
    )
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
