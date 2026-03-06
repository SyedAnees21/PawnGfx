use {
	crate::raster::RasterIn,
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
}
