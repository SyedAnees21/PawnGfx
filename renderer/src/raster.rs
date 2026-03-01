use {
	crate::{
		buffer::Buffers,
		shaders::{
			FragmentShader,
			GlobalUniforms,
			Varyings,
			VertexIn,
			VertexOut,
			VertexShader,
		},
	},
	pcore::{
		geometry::{Triangles, bounding_rect, edge_function},
		math::{self, Vector2, Vector4},
	},
	pscene::texture::{Albedo, NormalMap},
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

pub fn draw_call<VS, FS>(
	buffers: &mut Buffers,
	global_uniforms: &GlobalUniforms,
	albedo: &Albedo,
	normal: &NormalMap,
	triangles: Triangles,
	vs: &VS,
	fs: &FS,
) where
	VS: VertexShader,
	FS: FragmentShader,
{
	let w = global_uniforms.screen.width as i32;
	let h = global_uniforms.screen.height as i32;

	for v in triangles {
		let [v0, v1, v2] = v;

		let face_normal = (v1.position - v0.position)
			.cross(&(v2.position - v0.position))
			.normalize();

		let mut v_out = [VertexOut::default(); 3];

		for i in 0..3 {
			let v_in = VertexIn {
				attributes: v[i],
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

		// This block is applying:
		//
		// - Perspective division to clip space vertex
		// - Clip space to screen space transformation
		for i in 0..3 {
			let v_clip = v_out[i].clip;
			let inv_w = 1.0 / v_clip.w;

			let mut v_ndc = v_clip * inv_w;
			v_ndc.w = inv_w;

			r_vertices[i] = clip_to_screen(&v_ndc, w as f64, h as f64);
			varyings[i] = v_out[i].vary;
		}

		// Backface culling
		if is_backfacing(r_vertices[0].s, r_vertices[1].s, r_vertices[2].s) {
			continue;
		}

		// Perspective division:
		// uv, normal, tangents and varyings
		for i in 0..3 {
			varyings[i] = varyings[i] * r_vertices[i].inv_w;
		}

		draw_triangle_shaded(
			buffers,
			global_uniforms,
			albedo,
			normal,
			fs,
			varyings,
			r_vertices,
		);
	}
}

fn draw_triangle_shaded<FS>(
	// frame_buffer: &mut F,
	// depth_buffer: &mut D,
	buffers: &mut Buffers,
	global_uniforms: &GlobalUniforms,
	albedo: &Albedo,
	normal: &NormalMap,
	fs: &FS,
	varyings: [Varyings; 3],
	raster_in: [RasterIn; 3],
) where
	// F: AsMut<[u8]> + ?Sized,
	// D: AsMut<[f64]> + ?Sized,
	FS: FragmentShader,
{
	// let z_buffer = depth_buffer.as_mut();
	// let f_buffer = frame_buffer.as_mut();

	let (f_buffer, z_buffer) = buffers.mut_buffers();

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

			let is_outside = w0 < 0.0 || w1 < 0.0 || w2 < 0.0;

			if is_outside {
				continue;
			}

			let bary = (w0, w1, w2);

			let inv_depth =
				math::barycentric_interpolate(w0, w1, w2, inv_w0, inv_w1, inv_w2);
			let z = math::perspective_interpolate(bary, inv_depth, (z0, z1, z2));

			let depth_index = (y * w + x) as usize;
			let pixel_index = depth_index * 4;

			if z < z_buffer[depth_index] {
				let [v0, v1, v2] = varyings;

				let varying =
					math::perspective_interpolate(bary, inv_depth, (v0, v1, v2));
				let color = fs.shade(varying, global_uniforms, albedo, normal);

				z_buffer[depth_index] = z;
				f_buffer[pixel_index..pixel_index + 4]
					.copy_from_slice(&color.to_rgba8());
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
