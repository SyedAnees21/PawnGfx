use {
	crate::{
		buffer::Buffers,
		shaders::{
			FS, GVaryings, VS, Varyings, VertexIn, VertexOut, uniform::GlobalUniforms,
		},
	},
	pcore::{
		geometry::{IncEdge, bounding_rect, edge_function},
		math::{Gradient, Vector2, Vector4},
	},
	pscene::{object::ObjectRef, texture},
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
	uniforms: &mut GlobalUniforms,
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
			varyings[i] = shader.perspective_divide(varyings[i], &r_vertices[i]);
		}

		rasterize(buffers, object, uniforms, varyings, r_vertices, shader);
	}
}

pub fn rasterize<'d, S>(
	buffers: &mut Buffers,
	object: ObjectRef<'d>,
	uniforms: &mut GlobalUniforms,
	varyings: [Varyings; 3],
	raster_in: [RasterIn; 3],
	shader: &S,
) where
	S: FS,
{
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

	if min_x > max_x || min_y > max_y {
		return;
	}

	let screen = [s0, s1, s2];

	let dx = (min_x as f32 + 0.5) - s0.x;
	let dy = (min_y as f32 + 0.5) - s0.y;

	let g_varyings = shader.compute_gradients(varyings, screen, inv_area);
	let mut init_varying = shader.sample_gradients(&g_varyings, dx, dy);

	let g_inv_w = Gradient::new([inv_w0, inv_w1, inv_w2], screen, inv_area);
	let mut init_inv_w = g_inv_w.sample_at(dx, dy);

	let g_z = Gradient::new([z0, z1, z2], screen, inv_area);
	let mut init_z = g_z.sample_at(dx, dy);

	// Incremental edge function already normalized to screen
	// space triangle.
	let inc_edge = IncEdge::new(s0, s1, s2, Some(inv_area));
	let mut init_w = inc_edge.weights(min_x as f32 + 0.5, min_y as f32 + 0.5);

	for y in min_y..=max_y {
		let (mut w0, mut w1, mut w2) = init_w;

		let mut c_varyings = init_varying;
		let mut c_inv_w = init_inv_w;
		let mut c_z = init_z;

		let offset = (y * w + min_x) as usize;
		let mut buf_cursor = buffers.get_cursor(offset);

		for _ in min_x..=max_x {
			let is_outside = w0 < 0.0 || w1 < 0.0 || w2 < 0.0;

			if !is_outside && c_z < buf_cursor.get_depth() {
				let w_lerped = 1.0 / c_inv_w;

				let varyings = shader.recover_value(&c_varyings, w_lerped);

				lods(object, &g_varyings, &varyings, &g_inv_w, w_lerped, uniforms);

				let color = shader.shade_pixel(varyings, object, uniforms);

				buf_cursor.put_depth(c_z);
				buf_cursor.put_pixel(color);
			}

			(w0, w1, w2) = inc_edge.step_x(w0, w1, w2);

			shader.step_horizontal(&g_varyings, &mut c_varyings);
			g_inv_w.step_x(&mut c_inv_w);
			g_z.step_x(&mut c_z);

			buf_cursor.step();
		}

		init_w = inc_edge.step_y(init_w.0, init_w.1, init_w.2);
		shader.step_vertical(&g_varyings, &mut init_varying);
		g_inv_w.step_y(&mut init_inv_w);
		g_z.step_y(&mut init_z);
	}
}

fn lods<'d>(
	object: ObjectRef<'d>,
	g_varyings: &GVaryings,
	varyings: &Varyings,
	g_inv_w: &Gradient<f32>,
	w: f32,
	uniforms: &mut GlobalUniforms,
) {
	let _material = object.model.material;

	// // Inv depth
	// let inv_w = inv_w;
	// // Recovered depth
	// let w = 1.0 / inv_w;

	let inv_w_dx = g_inv_w.da_dx;
	let inv_w_dy = g_inv_w.da_dy;

	// Perspective correct UV coordinates
	let uv = varyings.uv;

	// Gradients of UV / w
	let uv_over_w_dx = g_varyings.uv.da_dx;
	let uv_over_w_dy = g_varyings.uv.da_dy;

	// Gradients of actual UV calculated by applying
	// Quotient Rule to get dU/dx, dV/dx, etc.
	// Formula: (dA - (A/B) * dB) / B
	let duv_dx = (uv_over_w_dx - uv * inv_w_dx) * w;
	let duv_dy = (uv_over_w_dy - uv * inv_w_dy) * w;

	// Note for now we are using the same lod due to same texture sizes
	// later on we need to optimize to sort at handle cases for similar
	// sizes.
	let lod = texture::sized_lod(1024.0, 1024.0, duv_dx, duv_dy);

	// if let Some(_) = material.albedo {
	// 	let lod = albedo.compute_lod(duv_dx, duv_dy);
	// 	uniforms.lods.albedo = Some(lod)
	// }

	// if let Some(_) = material.normal {
	// 	let lod = n_map.compute_lod(duv_dx, duv_dy);
	// 	uniforms.lods.normal = Some(lod)
	// }

	uniforms.lods.albedo = Some(lod);
	uniforms.lods.normal = Some(lod);
}

pub fn clip_to_screen(v_ndc: &Vector4, width: f32, height: f32) -> RasterIn {
	let screen_x = (v_ndc.x + 1.0) * 0.5 * width;
	let screen_y = (1.0 - (v_ndc.y + 1.0) * 0.5) * height;

	(Vector2::new(screen_x, screen_y), v_ndc.z, v_ndc.w).into()
}

pub fn is_backfacing(v0: Vector2, v1: Vector2, v2: Vector2) -> bool {
	edge_function(v0, v1, v2) < 0.0
}
