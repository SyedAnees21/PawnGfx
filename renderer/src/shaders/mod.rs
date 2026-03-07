use {
	crate::raster::RasterIn,
	pcore::math::Vector2,
	pscene::{color::Color, object::ObjectRef},
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

	fn sample_gradients(&self, g_varyings: &GVaryings, dx: f32, dy: f32) -> Varyings;
	fn recover_value(&self, varyings: &Varyings, inv_w: f32) -> Varyings;
	fn step_horizontal(&self, g_varyings: &GVaryings, varyings: &mut Varyings);
	fn step_vertical(&self, g_varyings: &GVaryings, varyings: &mut Varyings);
}
