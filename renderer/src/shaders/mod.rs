use {
	crate::raster::RasterIn,
	pcore::math::{Gradient, Vector2},
	pscene::{color::Color, object::ObjectRef, texture::Texture},
};
pub use {effects::*, io::*};

mod effects;
pub mod io;
pub mod uniform;

pub trait VS {
	fn shade_vertex<'d>(
		&self,
		input: VertexIn,
		object: ObjectRef<'d>,
		uniforms: &uniform::GlobalUniforms,
	) -> VertexOut;

	fn perspective_divide(
		&self,
		input: Varyings,
		raster_in: &RasterIn,
	) -> Varyings;
}

pub trait FS {
	fn shade_pixel<'d>(
		&self,
		input: Varyings,
		object: ObjectRef<'d>,
		uniforms: &uniform::GlobalUniforms,
	) -> Color;

	fn perspective_interpolate(
		&self,
		input: [Varyings; 3],
		bary: (f32, f32, f32),
		inv_depth: f32,
	) -> Varyings;

	fn compute_gradients(
		&self,
		varyings: [Varyings; 3],
		screen: [Vector2; 3],
		inv_det: f32,
	) -> GVaryings {
		GVaryings::from_varyings(varyings, screen, inv_det)
	}

	fn sample_gradients(
		&self,
		g_varyings: &GVaryings,
		dx: f32,
		dy: f32,
	) -> Varyings;

	fn recover_value(&self, varyings: &Varyings, inv_w: f32) -> Varyings;

	fn step_horizontal(&self, g_varyings: &GVaryings, varyings: &mut Varyings);

	fn step_vertical(&self, g_varyings: &GVaryings, varyings: &mut Varyings);
}

pub fn compute_lod<T>(
	current_inv_w: f32,
	gradient_inv_w: &Gradient<f32>,
	current_v: &Varyings,
	gradient_v: &GVaryings,
	texture: &Texture<T>,
) -> f32 {
	let (width, height) = texture.dimensions();

	let inv_w = current_inv_w;
	if current_inv_w.abs() < 1e-9 {
		return 0.0;
	}

	let inv_w2 = current_inv_w * current_inv_w;
	let w2 = 1.0 / inv_w2;

	// The Quotient Rule: d/dx (A/B) = (A'B - AB') / B^2
	// Here A = U_over_W, B = 1/W.
	// This recovers how much U changes relative to screen X.
	let du_dx = (gradient_v.uv.da_dx.x * inv_w
		- current_v.uv.x * gradient_inv_w.da_dx)
		/ w2;
	let dv_dx = (gradient_v.uv.da_dx.y * inv_w
		- current_v.uv.y * gradient_inv_w.da_dx)
		/ w2;

	let du_dy = (gradient_v.uv.da_dy.x * inv_w
		- current_v.uv.x * gradient_inv_w.da_dy)
		/ w2;
	let dv_dy = (gradient_v.uv.da_dy.y * inv_w
		- current_v.uv.y * gradient_inv_w.da_dy)
		/ w2;

	let dudx_scaled = du_dx * width as f32;
	let dvdx_scaled = dv_dx * height as f32;
	let dudy_scaled = du_dy * width as f32;
	let dvdy_scaled = dv_dy * height as f32;

	let len_x = dudx_scaled * dudx_scaled + dvdx_scaled * dvdx_scaled;
	let len_y = dudy_scaled * dudy_scaled + dvdy_scaled * dvdy_scaled;

	let quick_log2 = |val: f32| -> f32 {
		let bits = val.to_bits();
		let exp = ((bits >> 23) & 0xff) as i32 - 127;
		let mant = (bits & 0x7fffff) as f32 / 8388608.0;

		exp as f32 + mant
	};

	// let rho = len_x.max(len_y).max(1e-8);
	// let lod = 0.5 * rho.log2();

	let rho = len_x.max(len_y).max(1e-8);
	let lod = 0.5 * quick_log2(rho);

	lod.clamp(0.0, texture.max_lod())
}
