use {
	crate::{
		buffer::Buffers,
		shaders::{FS, VS, Varyings, VertexIn, VertexOut, uniform},
	},
	pcore::{
		geometry::{IncEdge, bounding_rect, edge_function},
		math::{self, Vector2, Vector4},
	},
	pscene::object::ObjectRef,
};

#[derive(Default, Clone, Copy)]
pub struct RasterIn {
	pub s: Vector2,
	pub z: f32,
	pub inv_w: f32,
}

impl From<(Vector2, f32, f32)> for RasterIn {
	fn from(value: (Vector2, f32, f32)) -> Self {
		let (screen, z, inv_w) = value;
		Self {
			s: screen,
			z,
			inv_w,
		}
	}
}

pub fn consume_draw_call<'d, S>(
	buffers: &mut Buffers,
	object: ObjectRef<'d>,
	uniforms: &uniform::GlobalUniforms,
	shader: &S,
) where
	S: VS + FS,
{
	let w = uniforms.screen.width as i32;
	let h = uniforms.screen.height as i32;

	for v in object.model.mesh.iter_triangles() {
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

			v_out[i] = shader.shade_vertex(v_in, object, uniforms);
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

			r_vertices[i] = clip_to_screen(&v_ndc, w as f32, h as f32);
			varyings[i] = v_out[i].vary;
		}

		// Backface culling
		if is_backfacing(r_vertices[0].s, r_vertices[1].s, r_vertices[2].s) {
			continue;
		}

		// Perspective division:
		// uv, normal, tangents and varyings
		for i in 0..3 {
			// varyings[i] = varyings[i] * r_vertices[i].inv_w;
			varyings[i] = shader.perspective_divide(varyings[i], &r_vertices[i]);
		}

		rasterize(buffers, object, uniforms, varyings, r_vertices, shader);
	}
}

pub fn rasterize<'d, S>(
	buffers: &mut Buffers,
	object: ObjectRef<'d>,
	uniforms: &uniform::GlobalUniforms,
	varyings: [Varyings; 3],
	raster_in: [RasterIn; 3],
	shader: &S,
) where
	S: FS,
{
	let (f_buffer, z_buffer) = buffers.mut_buffers();

	let w = uniforms.screen.width as i32;
	let h = uniforms.screen.height as i32;

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
	let max_x = max.x.min((w - 1) as f32) as i32;
	let max_y = max.y.min((h - 1) as f32) as i32;

	// Incremental edge function already normalized to screen
	// space triangle.
	let inc_edge = IncEdge::new(s0, s1, s2, Some(inv_area));
	let mut init_w = inc_edge.weights(min_x as f32 + 0.5, min_y as f32 + 0.5);

	for y in min_y..=max_y {
		let (mut w0, mut w1, mut w2) = init_w;

		for x in min_x..=max_x {
			let is_outside = w0 < 0.0 || w1 < 0.0 || w2 < 0.0;

			if is_outside {
				(w0, w1, w2) = inc_edge.step_x(w0, w1, w2);
				continue;
			}

			let bary = (w0, w1, w2);

			let inv_depth =
				math::barycentric_interpolate(w0, w1, w2, inv_w0, inv_w1, inv_w2);
			let z = math::perspective_interpolate(bary, inv_depth, (z0, z1, z2));

			let depth_index = (y * w + x) as usize;
			let pixel_index = depth_index * 4;

			if z < z_buffer[depth_index] {
				let varying = shader.perspective_interpolate(varyings, bary, inv_depth);
				let color = shader.shade_pixel(varying, object, uniforms);

				z_buffer[depth_index] = z;
				f_buffer[pixel_index..pixel_index + 4]
					.copy_from_slice(&color.to_rgba8());
			}

			(w0, w1, w2) = inc_edge.step_x(w0, w1, w2);
		}

		init_w = inc_edge.step_y(init_w.0, init_w.1, init_w.2);
	}
}

pub fn clip_to_screen(v_ndc: &Vector4, width: f32, height: f32) -> RasterIn {
	let screen_x = (v_ndc.x + 1.0) * 0.5 * width;
	let screen_y = (1.0 - (v_ndc.y + 1.0) * 0.5) * height;

	(Vector2::new(screen_x, screen_y), v_ndc.z, v_ndc.w).into()
}

pub fn is_backfacing(v0: Vector2, v1: Vector2, v2: Vector2) -> bool {
	edge_function(v0, v1, v2) < 0.0
}
