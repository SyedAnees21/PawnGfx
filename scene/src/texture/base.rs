use {
	crate::texture::TextureSampler,
	image::Rgb,
	pcore::{error::PResult, geometry::UV, math::Arithmetic},
	std::path::Path,
};

const TILE_SIZE: usize = 4; // Because we are using 4x4 tiles
const TILE_SHIFT: usize = 2; // Because 2^2 = 4
const TILE_MASK: usize = TILE_SIZE - 1; // Because 4 - 1 = 3

pub enum Texture<T, S>
where
	S: TextureSampler<Out = T>,
	T: Copy + Arithmetic,
{
	Constant(T),
	Map(S),
}

impl<T, S> Texture<T, S>
where
	S: TextureSampler<Out = T>,
	T: Copy + Arithmetic,
{
	pub fn constant(val: T) -> Self {
		Self::Constant(val)
	}

	pub fn map(map: S) -> Self {
		Self::Map(map)
	}

	#[inline]
	pub fn is_constant(&self) -> bool {
		matches!(self, Texture::Constant(_))
	}

	#[inline]
	pub fn is_map(&self) -> bool {
		matches!(self, Texture::Map(_))
	}

	#[inline]
	pub fn sample(&self, u: f32, v: f32, lod: f32) -> T {
		match self {
			Texture::Constant(val) => *val,
			Texture::Map(map) => map.sample(u, v, lod),
		}
	}

	#[inline]
	pub fn bi_sample(&self, u: f32, v: f32, lod: f32) -> T {
		match self {
			Texture::Constant(val) => *val,
			Texture::Map(map) => map.bi_sample(u, v, lod),
		}
	}

	#[inline]
	pub fn tri_sample(&self, u: f32, v: f32, lod: f32) -> T {
		match self {
			Texture::Constant(val) => *val,
			Texture::Map(map) => map.tri_sample(u, v, lod),
		}
	}
}

#[derive(Clone, Copy)]
pub enum Wrap {
	Clamp,
	Repeat,
	Mirror,
}

pub struct TextureMap<T> {
	pub mipmap: Vec<Mip<T>>,
	pub wrap: Wrap,
}

impl<T> Default for TextureMap<T> {
	fn default() -> Self {
		Self {
			mipmap: vec![],
			wrap: Wrap::Clamp,
		}
	}
}

impl<T> TextureMap<T> {
	pub fn new(w: usize, h: usize, data: Vec<T>, wrap_mode: Wrap) -> Self {
		Self {
			mipmap: vec![Mip::new(w, h, data)],
			wrap: wrap_mode,
		}
	}

	pub fn get_level(&self, lod: usize) -> &Mip<T> {
		let level = lod;
		&self.mipmap[level]
	}

	#[inline(always)]
	pub fn lod(&self, duv_dx: UV, duv_dy: UV) -> f32 {
		let (w, h) = self.dimensions();

		// 3. Scale by texture size to get Texel Gradients
		let du_dx_s = duv_dx.x * w as f32;
		let dv_dx_s = duv_dx.y * h as f32;
		let du_dy_s = duv_dy.x * w as f32;
		let dv_dy_s = duv_dy.y * h as f32;

		// 4. Calculate squared lengths of the pixel footprints
		let len_x_sq = du_dx_s * du_dx_s + dv_dx_s * dv_dx_s;
		let len_y_sq = du_dy_s * du_dy_s + dv_dy_s * dv_dy_s;

		// 5. LOD = log2(max(len_x, len_y))
		// Which is the same as: 0.5 * log2(max(len_x_sq, len_y_sq))
		let rho_sq = len_x_sq.max(len_y_sq).max(1e-8);
		0.5 * rho_sq.log2()
	}

	#[inline(always)]
	pub fn unsafe_get_level(&self, lod: usize) -> &Mip<T> {
		// SAFETY: Caller must make sure the the LOD is clamped
		// This should be done in the texture sampling stage.
		unsafe { self.mipmap.get_unchecked(lod) }
	}

	#[inline(always)]
	pub fn wrap_uv(&self, p: f32) -> f32 {
		match self.wrap {
			// Clamp: Clamps the out of bounds value to the nearest edge
			Wrap::Clamp => p.clamp(0.0, 1.0),

			// Repeat: p - floor(p) handles negative and positive correctly
			Wrap::Repeat => p - p.floor(),

			// Mirror: 1.0 - abs( (p-1) % 2 - 1 )
			Wrap::Mirror => {
				let base = (p - 1.0) - ((p - 1.0) * 0.5).floor() * 2.0;
				(base - 1.0).abs()
			}
		}
	}

	#[inline]
	pub fn size(&self) -> usize {
		let base = &self.mipmap[0];
		base.width.max(base.height)
	}

	#[inline]
	pub fn dimensions(&self) -> (usize, usize) {
		let base = &self.mipmap[0];
		(base.width, base.height)
	}

	#[inline]
	pub fn is_squared_size(&self) -> bool {
		let base = &self.mipmap[0];
		base.width == base.height
	}

	#[inline]
	pub fn max_lod(&self) -> f32 {
		self.mipmap.len().saturating_sub(1) as f32
	}

	#[inline]
	pub fn clamp_lod(&self, lod: f32) -> f32 {
		lod.clamp(0.0, self.max_lod())
	}
}

impl<T> TextureMap<T> {
	pub fn from_file<P, C, A>(
		path: P,
		wrap_mode: Wrap,
		converter: C,
		averager: A,
	) -> PResult<Self>
	where
		P: AsRef<Path>,
		T: Copy + Arithmetic + Default,
		C: Fn(Rgb<u8>) -> T,
		A: Fn(T, T, T, T) -> T,
	{
		let img = image::open(path)?.to_rgb8();
		let (w, h) = img.dimensions();

		let mut data = Vec::with_capacity((w * h) as usize);

		for pixel in img.pixels() {
			let texel = converter(*pixel);
			data.push(texel);
		}

		let mut tex = Self::new(w as usize, h as usize, data, wrap_mode);
		tex.bake(averager);

		Ok(tex)
	}

	pub fn bake<A>(&mut self, averager: A)
	where
		T: Copy + Arithmetic + Default,
		A: Fn(T, T, T, T) -> T,
	{
		let mut level = 0;
		loop {
			let base = &self.mipmap[level];

			if base.width <= 1 && base.height <= 1 {
				break;
			}

			let new_w = (base.width / 2).max(1);
			let new_h = (base.height / 2).max(1);

			let mut new_data = vec![T::default(); new_w * new_h];

			let w = base.width - 1;
			let h = base.height - 1;

			for y in 0..new_h {
				let y_off = 2 * y;

				for x in 0..new_w {
					let x_off = 2 * x;

					let x0 = (x_off).min(w);
					let x1 = (x_off + 1).min(w);

					let y0 = (y_off).min(h);
					let y1 = (y_off + 1).min(h);

					let c0 = base.unsafe_texel(x0, y0);
					let c1 = base.unsafe_texel(x1, y0);
					let c2 = base.unsafe_texel(x0, y1);
					let c3 = base.unsafe_texel(x1, y1);

					new_data[y * new_w + x] = averager(c0, c1, c2, c3);
				}
			}

			self.mipmap.push(Mip::new(new_w, new_h, new_data));
			level += 1;
		}
	}
}

#[derive(Clone)]
pub struct Mip<T> {
	pub(crate) width: usize,
	pub(crate) height: usize,
	pub(crate) x_tiles: usize,
	pub(crate) data: Vec<T>,
}

impl<T> Mip<T> {
	pub fn new(width: usize, height: usize, data: Vec<T>) -> Self {
		// Note: This is a safety measure to make sure that the memory
		// is never violated while doing unchecked texel access.
		assert!(
			data.len() == width * height,
			"Mismatched dimension to the data"
		);

		Self {
			width,
			height,
			x_tiles: 0,
			data,
		}
	}

	#[inline]
	pub fn index(x: usize, y: usize, x_tiles: usize) -> usize {
		// x / TILE_SIZE, y / TILE_SIZE
		let tile_x = x >> TILE_SHIFT;
		let tile_y = y >> TILE_SHIFT;

		// x % TILE_SIZE, y % TILE_SIZE
		let local_x = x & TILE_MASK;
		let local_y = y & TILE_MASK;

		// local_y * TILE_SIZE + local_x
		let in_tile = local_y << TILE_SHIFT | local_x;
		let tile_index = tile_y * x_tiles + tile_x;

		// tile_index * TILE_SIZE^2 + in_tile
		(tile_index << (TILE_SHIFT * 2)) + in_tile
	}

	#[inline]
	pub fn unsafe_texel(&self, u: usize, v: usize) -> T
	where
		T: Copy,
	{
		// SAFETY: We have tried to made sure at the time of
		// mipmap creation the the data boundary is long enough
		// to not go out of bounds and we expect that the caller
		// will wrap the UVs responsibly.
		unsafe {
			let index = v * self.width + u;
			*self.data.get_unchecked(index)
		}
	}

	#[inline]
	pub fn texel(&self, u: usize, v: usize) -> T
	where
		T: Copy,
	{
		let index = Self::index(u, v, self.x_tiles);
		self.data[index]
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::{Albedo, Wrap};

// 	#[test]
// 	fn load_checker_texture() {
// 		let path = "../assets/texture/Checker-Texture.png";
// 		let texture = Albedo::load(path, Wrap::Clamp).unwrap();

// 		let (width, height) = texture.dimensions();
// 		assert_eq!(width, 1024);
// 		assert_eq!(height, 1024);
// 	}
// }
