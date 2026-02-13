use crate::{
    geometry::{Triangles, bounding_rect, edge_function},
    math::{self, Vector2, Vector4},
    scene::Texture,
    shaders::{
        FragmentShader, GlobalUniforms, Varyings, VertexAttributes, VertexIn, VertexOut,
        VertexShader,
    },
};

#[derive(Default, Clone, Copy)]
pub struct RasterIn {
    pub s: Vector2,
    pub z: f64,
    pub inv_w: f64,
}

impl From<(Vector2, f64, f64)> for RasterIn {
    fn from(value: (Vector2, f64, f64)) -> Self {
        let (screen, z, inv_w) = value;
        Self {
            s: screen,
            z,
            inv_w,
        }
    }
}

pub fn draw_call<F, D, VS, FS>(
    frame_buffer: &mut F,
    depth_buffer: &mut D,
    global_uniforms: &GlobalUniforms,
    texture: &Texture,
    triangles: Triangles,
    vs: &VS,
    fs: &FS,
) where
    F: AsMut<[u8]> + ?Sized,
    D: AsMut<[f64]> + ?Sized,
    VS: VertexShader,
    FS: FragmentShader,
{
    let w = global_uniforms.screen.width as i32;
    let h = global_uniforms.screen.height as i32;

    for (v, n, uv) in triangles {
        let [v0, v1, v2] = v;

        let face_normal = (v1 - v0).cross(&(v2 - v0)).normalize();

        let mut v_out = [VertexOut::default(); 3];

        for i in 0..3 {
            let v_in = VertexIn {
                attributes: VertexAttributes {
                    position: v[i],
                    normal: n[i],
                    uv: uv[i],
                },
                face_normal,
            };

            v_out[i] = vs.shade(v_in, global_uniforms);
        }

        let outside_clip_space = v_out.iter().any(|out| out.clip.w <= 0.0);

        if outside_clip_space {
            continue;
        }

        let mut r_vertices = [RasterIn::default(); 3];
        let mut varyings = [Varyings::default(); 3];

        for i in 0..3 {
            let v_clip = v_out[i].clip;
            let inv_w = 1.0 / v_clip.w;

            let mut v_ndc = v_clip * inv_w;
            v_ndc.w = inv_w;

            r_vertices[i] = clip_to_screen(&v_ndc, w as f64, h as f64);
            varyings[i] = v_out[i].vary;
        }

        if is_backfacing(r_vertices[0].s, r_vertices[1].s, r_vertices[2].s) {
            continue;
        }

        for i in 0..3 {
            varyings[i] = varyings[i] * r_vertices[i].inv_w;
        }

        draw_triangle_shaded(
            frame_buffer,
            depth_buffer,
            global_uniforms,
            texture,
            fs,
            varyings,
            r_vertices,
        );
    }
}

fn draw_triangle_shaded<F, D, FS>(
    frame_buffer: &mut F,
    depth_buffer: &mut D,
    global_uniforms: &GlobalUniforms,
    texture: &Texture,
    fs: &FS,
    varyings: [Varyings; 3],
    raster_in: [RasterIn; 3],
) where
    F: AsMut<[u8]> + ?Sized,
    D: AsMut<[f64]> + ?Sized,
    FS: FragmentShader,
{
    let depth_buffer = depth_buffer.as_mut();
    let frame_buffer = frame_buffer.as_mut();

    let w = global_uniforms.screen.width as i32;
    let h = global_uniforms.screen.height as i32;

    let [
        RasterIn {
            s: s0,
            z: z0,
            inv_w: inv_w0,
        },
        RasterIn {
            s: s1,
            z: z1,
            inv_w: inv_w1,
        },
        RasterIn {
            s: s2,
            z: z2,
            inv_w: inv_w2,
        },
    ] = raster_in;

    let [v0, v1, v2] = varyings;

    let area = edge_function(s0, s1, s2);
    if area.abs() < 1e-9 {
        return;
    }
    let inv_area = 1.0 / area;

    let (du_dx, du_dy) = attr_gradient(v0.uv.x, v1.uv.x, v2.uv.x, s0, s1, s2, inv_area);
    let (dv_dx, dv_dy) = attr_gradient(v0.uv.y, v1.uv.y, v2.uv.y, s0, s1, s2, inv_area);
    let (dw_dx, dw_dy) = attr_gradient(inv_w0, inv_w1, inv_w2, s0, s1, s2, inv_area);

    let (tex_w, tex_h) = texture.dimensions();
    let tex_w = tex_w as f64;
    let tex_h = tex_h as f64;
    let max_lod = texture.max_lod();

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

            let is_outside = w0 < 0.0 || w1 < 0.0 || w2 < 0.0;

            if is_outside {
                continue;
            }

            let bary = (w0, w1, w2);

            let inv_depth = math::barycentric_interpolate(w0, w1, w2, inv_w0, inv_w1, inv_w2);
            let z = math::perspective_interpolate(bary, inv_depth, (z0, z1, z2));

            let depth_index = (y * w + x) as usize;
            let pixel_index = depth_index * 4;

            if z < depth_buffer[depth_index] {
                let varying = math::perspective_interpolate(bary, inv_depth, (v0, v1, v2));

                let u_prime = math::barycentric_interpolate(w0, w1, w2, v0.uv.x, v1.uv.x, v2.uv.x);
                let v_prime = math::barycentric_interpolate(w0, w1, w2, v0.uv.y, v1.uv.y, v2.uv.y);

                let lod = lod_from_derivatives(
                    u_prime,
                    v_prime,
                    inv_depth,
                    du_dx,
                    du_dy,
                    dv_dx,
                    dv_dy,
                    dw_dx,
                    dw_dy,
                    tex_w,
                    tex_h,
                    max_lod,
                );

                let color = fs.shade(varying, global_uniforms, texture, lod);

                depth_buffer[depth_index] = z;
                frame_buffer[pixel_index..pixel_index + 4].copy_from_slice(&color.to_rgba8());
            }
        }
    }
}

pub fn clip_to_screen(v_ndc: &Vector4, width: f64, height: f64) -> RasterIn {
    let screen_x = (v_ndc.x + 1.0) * 0.5 * width;
    let screen_y = (1.0 - (v_ndc.y + 1.0) * 0.5) * height;

    (Vector2::new(screen_x, screen_y), v_ndc.z, v_ndc.w).into()
}

pub fn is_backfacing(v0: Vector2, v1: Vector2, v2: Vector2) -> bool {
    edge_function(v0, v1, v2) < 0.0
}

fn attr_gradient(
    f0: f64,
    f1: f64,
    f2: f64,
    s0: Vector2,
    s1: Vector2,
    s2: Vector2,
    inv_area: f64,
) -> (f64, f64) {
    let dy20 = s2.y - s0.y;
    let dy10 = s1.y - s0.y;
    let dx20 = s2.x - s0.x;
    let dx10 = s1.x - s0.x;

    let df10 = f1 - f0;
    let df20 = f2 - f0;

    let df_dx = (df10 * dy20 - df20 * dy10) * inv_area;
    let df_dy = (-df10 * dx20 + df20 * dx10) * inv_area;

    (df_dx, df_dy)
}

fn lod_from_derivatives(
    u_prime: f64,
    v_prime: f64,
    inv_w: f64,
    du_dx: f64,
    du_dy: f64,
    dv_dx: f64,
    dv_dy: f64,
    dw_dx: f64,
    dw_dy: f64,
    tex_w: f64,
    tex_h: f64,
    max_lod: f64,
) -> f64 {
    if inv_w.abs() < 1e-9 {
        return 0.0;
    }

    let inv_w2 = inv_w * inv_w;
    let dux = (du_dx * inv_w - u_prime * dw_dx) / inv_w2;
    let duy = (du_dy * inv_w - u_prime * dw_dy) / inv_w2;
    let dvx = (dv_dx * inv_w - v_prime * dw_dx) / inv_w2;
    let dvy = (dv_dy * inv_w - v_prime * dw_dy) / inv_w2;

    let dudx = dux * tex_w;
    let dvdx = dvx * tex_h;
    let dudy = duy * tex_w;
    let dvdy = dvy * tex_h;

    let len_x = (dudx * dudx + dvdx * dvdx).sqrt();
    let len_y = (dudy * dudy + dvdy * dvdy).sqrt();

    let rho = len_x.max(len_y).max(1e-8);
    let lod = rho.log2();

    lod.clamp(0.0, max_lod)
}
