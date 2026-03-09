use {
	crate::{assets::loader::AssetLoader, color::Color},
	image::Rgb,
	pcore::{
		error::PResult,
		geometry::Normal,
		math::{self, Arithmetic},
	},
	std::path::Path,
};

const TILE_SIZE: usize = 4; // Because we are using 4x4 tiles
const TILE_SHIFT: usize = 2; // Because 2^2 = 4
const TILE_MASK: usize = 3; // Because 4 - 1 = 3

#[derive(Clone)]
pub struct Mip<T> {
	width: usize,
	height: usize,
	x_tiles: usize,
	data: Vec<T>,
}

impl<T> Mip<T> {
	pub fn new(width: usize, height: usize, data: Vec<T>) -> Self {
		let x_tiles = (width + TILE_SIZE - 1) / TILE_SIZE;
		let y_tiles = (height + TILE_SIZE - 1) / TILE_SIZE;
		let padded_w = x_tiles * TILE_SIZE;
		let padded_h = y_tiles * TILE_SIZE;

		// Note: This is a safety measure to make sure that the memory
		// is never violated while doing unchecked texel access.
		assert!(
			data.len() == padded_w * padded_h,
			"Mismatched padded tiles dimension to the data"
		);

		Self {
			width,
			height,
			x_tiles,
			data,
		}
	}

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
		let index = (tile_index << (TILE_SHIFT * 2)) + in_tile;

		index
	}

	pub fn unsafe_texel(&self, u: usize, v: usize) -> T
	where
		T: Copy,
	{
		// SAFETY: We have tried to made sure at the time of
		// mipmap creation the the data boundary is long enough
		// to not go out of bounds and we expect that the caller
		// will wrap the UVs responsibly.
		unsafe {
			let index = Self::index(u, v, self.x_tiles);
			*self.data.get_unchecked(index)
		}
	}

	pub fn texel(&self, u: usize, v: usize) -> T
	where
		T: Copy,
	{
		let index = Self::index(u, v, self.x_tiles);
		self.data[index]
	}

	pub fn sample(&self, u: f32, v: f32) -> T
	where
		T: Copy,
	{
		let x = (u * (self.width as f32 - 1.0)) as usize;
		let y = (v * (self.height as f32 - 1.0)) as usize;

		self.unsafe_texel(x, y)
	}

	pub fn bi_sample(&self, u: f32, v: f32) -> T
	where
		T: Copy + Arithmetic,
	{
		let x = u * (self.width as f32 - 1.0);
		let y = v * (self.height as f32 - 1.0);

		let x0 = x.floor() as usize;
		let y0 = y.floor() as usize;
		let x1 = (x0 + 1).min(self.width - 1);
		let y1 = (y0 + 1).min(self.height - 1);

		let tx = x - x.floor();
		let ty = y - y.floor();

		let c00 = self.unsafe_texel(x0, y0);
		let c10 = self.unsafe_texel(x1, y0);
		let c01 = self.unsafe_texel(x0, y1);
		let c11 = self.unsafe_texel(x1, y1);

		math::bi_lerp(c00, c01, c10, c11, tx, ty)
	}
}

pub enum Wrap {
	Clamp,
	Repeat,
	Mirror,
}

pub struct TextureMap<T> {
	mipmap: Vec<Mip<T>>,
	wrap: Wrap,
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

		let w = w as usize;
		let h = h as usize;

		let x_tiles = (w + TILE_SIZE - 1) / TILE_SIZE;
		let y_tiles = (h + TILE_SIZE - 1) / TILE_SIZE;

		let padded_w = x_tiles * TILE_SIZE;
		let padded_h = y_tiles * TILE_SIZE;

		let mut data = vec![T::default(); padded_w * padded_h];

		for y in 0..h {
			for x in 0..w {
				let pixel = img.get_pixel(x as u32, y as u32);
				let texel = converter(*pixel);

				let tile_x = x / TILE_SIZE;
				let tile_y = y / TILE_SIZE;

				let local_x = x % TILE_SIZE;
				let local_y = y % TILE_SIZE;

				let tile_index = tile_y * x_tiles + tile_x;
				let in_tile = local_y * TILE_SIZE + local_x;

				let index = tile_index * (TILE_SIZE * TILE_SIZE) + in_tile;
				data[index] = texel;
			}
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
		let mut level: usize = 0;

		loop {
			let base = &self.mipmap[level];

			if base.width <= 1 && base.height <= 1 {
				break;
			}

			let new_w = (base.width / 2).max(1);
			let new_h = (base.height / 2).max(1);

			let x_tiles = (new_w + TILE_SIZE - 1) / TILE_SIZE;
			let y_tiles = (new_h + TILE_SIZE - 1) / TILE_SIZE;

			let padded_w = x_tiles * TILE_SIZE;
			let padded_h = y_tiles * TILE_SIZE;

			let mut new_data = vec![T::default(); padded_w * padded_h];

			for y in 0..new_h {
				for x in 0..new_w {
					let x0 = (2 * x).min(base.width - 1);
					let x1 = (2 * x + 1).min(base.width - 1);
					let y0 = (2 * y).min(base.height - 1);
					let y1 = (2 * y + 1).min(base.height - 1);

					let c0 = base.unsafe_texel(x0, y0);
					let c1 = base.unsafe_texel(x1, y0);
					let c2 = base.unsafe_texel(x0, y1);
					let c3 = base.unsafe_texel(x1, y1);

					let avg_texel = averager(c0, c1, c2, c3);
					let index = Mip::<T>::index(x, y, x_tiles);

					new_data[index] = avg_texel;
				}
			}

			self.mipmap.push(Mip::new(new_w, new_h, new_data));
			level += 1;
		}
	}

	pub fn get_level(&self, lod: usize) -> &Mip<T> {
		let level = lod.min(self.mipmap.len() - 1);
		&self.mipmap[level]
	}

	pub fn wrap_uv(&self, mut p: f32) -> f32 {
		match self.wrap {
			Wrap::Clamp => p.clamp(0.0, 1.0),
			Wrap::Repeat => {
				p = p.fract();
				if p < 0.0 {
					p += 1.0;
				}
				p
			}
			Wrap::Mirror => {
				let mut p = p % 2.0;
				if p < 0.0 {
					p += 2.0;
				}
				if p > 1.0 { 2.0 - p } else { p }
			}
		}
	}

	pub fn sample(&self, mut u: f32, mut v: f32, lod: usize) -> T
	where
		T: Copy,
	{
		u = self.wrap_uv(u);
		v = self.wrap_uv(v);

		// Flipping image space
		v = 1.0 - v;

		self.get_level(lod).sample(u, v)
	}

	pub fn bi_sample(&self, mut u: f32, mut v: f32, lod: usize) -> T
	where
		T: Copy + Arithmetic,
	{
		u = self.wrap_uv(u);
		v = self.wrap_uv(v);

		// Flipping image space
		v = 1.0 - v;

		self.get_level(lod).bi_sample(u, v)
	}

	pub fn tri_sample(&self, mut u: f32, mut v: f32, lod: f32) -> T
	where
		T: Copy + Arithmetic,
	{
		let lod = self.clamp_lod(lod);
		let l0 = lod.floor() as usize;
		let l1 = (l0 + 1).min(self.mipmap.len() - 1);
		let t = lod.fract();

		u = self.wrap_uv(u);
		v = self.wrap_uv(v);

		v = 1.0 - v;

		let c_0 = self.get_level(l0).bi_sample(u, v);
		let c_1 = self.get_level(l1).bi_sample(u, v);

		math::lerp(c_0, c_1, t)
	}

	pub fn size(&self) -> usize {
		let base = &self.mipmap[0];
		base.width.max(base.height)
	}

	pub fn dimensions(&self) -> (usize, usize) {
		let base = &self.mipmap[0];
		(base.width, base.height)
	}

	pub fn max_lod(&self) -> f32 {
		if self.mipmap.is_empty() {
			0.0
		} else {
			(self.mipmap.len() - 1) as f32
		}
	}

	pub fn clamp_lod(&self, lod: f32) -> f32 {
		lod.clamp(0.0, self.max_lod())
	}
}

impl<T> TextureMap<T> {
	#[allow(deprecated)]
	#[allow(unused)]
	#[deprecated = "These functions are not going to be used any longer but just kept for early memories :)"]
	fn deprecated_from_file<P>(path: P, wrap_mode: Wrap) -> PResult<Self>
	where
		P: AsRef<Path>,
		T: From<Rgb<u8>> + Copy + Arithmetic + Default,
	{
		let img = image::open(path)?.to_rgb8();
		let (w, h) = img.dimensions();

		let mut data = Vec::with_capacity((w * h) as usize);

		for pixel in img.pixels() {
			data.push(T::from(*pixel));
		}

		let mut tex = Self::new(w as usize, h as usize, data, wrap_mode);
		tex.deprecated_bake();

		Ok(tex)
	}

	#[allow(unused)]
	#[deprecated = "These functions are not going to be used any longer but just kept for early memories :)"]
	fn deprecated_bake(&mut self)
	where
		T: Copy + Arithmetic + Default,
	{
		let mut base = &self.mipmap[0];

		while base.width > 1 || base.height > 1 {
			let new_w = (base.width / 2).max(1);
			let new_h = (base.height / 2).max(1);

			let mut new_data = vec![T::default(); new_w * new_h];

			for y in 0..new_h {
				for x in 0..new_w {
					let x0 = (2 * x).min(base.width - 1);
					let x1 = (2 * x + 1).min(base.width - 1);
					let y0 = (2 * y).min(base.height - 1);
					let y1 = (2 * y + 1).min(base.height - 1);

					let c0 = base.unsafe_texel(x0, y0);
					let c1 = base.unsafe_texel(x1, y0);
					let c2 = base.unsafe_texel(x0, y1);
					let c3 = base.unsafe_texel(x1, y1);

					new_data[y * new_w + x] = (c0 + c1 + c2 + c3) * 0.25;
					// c0.add_raw(c1).add_raw(c2).add_raw(c3) * 0.25;
				}
			}

			let next = Mip::new(new_w, new_h, new_data);
			self.mipmap.push(next);
			base = self.mipmap.last().unwrap();
		}
	}
}
pub enum Texture<T> {
	Constant(T),
	Map(TextureMap<T>),
}

impl<T> Texture<T>
where
	T: Copy + Arithmetic,
{
	pub fn constant(val: T) -> Self {
		Self::Constant(val)
	}

	pub fn map(map: TextureMap<T>) -> Self {
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
			Texture::Map(map) => map.sample(u, v, lod as usize),
		}
	}

	#[inline]
	pub fn bi_sample(&self, u: f32, v: f32, lod: f32) -> T {
		match self {
			Texture::Constant(val) => *val,
			Texture::Map(map) => map.bi_sample(u, v, lod as usize),
		}
	}
}

pub type Albedo = TextureMap<Color>;

// impl From<Rgb<u8>> for Color {
// 	fn from(value: Rgb<u8>) -> Self {
// 		Color::new_rgb(
// 			value[0] as f32 / 255.0,
// 			value[1] as f32 / 255.0,
// 			value[2] as f32 / 255.0,
// 		)
// 	}
// }

impl Albedo {
	pub fn load<P>(path: P, wrap: Wrap) -> PResult<Self>
	where
		P: AsRef<Path>,
	{
		let converter = |p: Rgb<u8>| -> Color {
			Color::new_rgb(
				p[0] as f32 / 255.0,
				p[1] as f32 / 255.0,
				p[2] as f32 / 255.0,
			)
		};

		let averager = |c0: Color, c1: Color, c2: Color, c3: Color| -> Color {
			(c0 + c1 + c2 + c3) * 0.25
		};

		TextureMap::<Color>::from_file(path, wrap, converter, averager)
	}

	// #[inline(always)]
	// // pub fn bi_sample(&self, mut u: f32, mut v: f32) -> Color {
	// // 	u = self.wrap_uv(u);
	// // 	v = self.wrap_uv(v);

	// // 	// Flipping image space
	// // 	v = 1.0 - v;

	// // 	let x = u * (self.width as f32 - 1.0);
	// // 	let y = v * (self.height as f32 - 1.0);

	// // 	let x0 = x.floor() as usize;
	// // 	let y0 = y.floor() as usize;
	// // 	let x1 = (x0 + 1).min(self.width - 1);
	// // 	let y1 = (y0 + 1).min(self.height - 1);

	// // 	let tx = x - x.floor();
	// // 	let ty = y - y.floor();

	// // 	let c00 = self.texel(x0, y0);
	// // 	let c10 = self.texel(x1, y0);
	// // 	let c01 = self.texel(x0, y1);
	// // 	let c11 = self.texel(x1, y1);

	// // 	math::bi_lerp(c00, c01, c10, c11, tx, ty)
	// // }
}

// impl AssetLoader for Albedo {
// // 	type Args = Wrap;

// // 	fn load_from_file<P>(path: P, args: Self::Args) -> PResult<Self>
// // 	where
// // 		P: AsRef<Path>,
// // 		Self: Sized,
// // 	{
// // 		let img = image::open(path)?.to_rgb8();
// // 		let (w, h) = img.dimensions();

// // 		let mut data = Vec::with_capacity((w * h) as usize);

// // 		for pixel in img.pixels() {
// // 			data.push(Color::from(*pixel))
// // 		}

// // 		Ok(Self {
// // 			width: w as usize,
// // 			height: h as usize,
// // 			wrap: args,
// // 			data,
// // 		})
// // 	}
// // }

pub type NormalMap = TextureMap<Normal>;

impl NormalMap {
	pub fn load<P>(path: P, wrap: Wrap) -> PResult<Self>
	where
		P: AsRef<Path>,
	{
		let converter = |p: Rgb<u8>| -> Normal {
			Normal::new(
				p[0] as f32 / 255.0 * 2.0 - 1.0,
				p[1] as f32 / 255.0 * 2.0 - 1.0,
				p[2] as f32 / 255.0 * 2.0 - 1.0,
			)
		};

		let averager = |c0: Normal, c1: Normal, c2: Normal, c3: Normal| -> Normal {
			((c0 + c1 + c2 + c3) * 0.25).normalize()
		};

		TextureMap::<Normal>::from_file(path, wrap, converter, averager)
	}
}
// impl From<Rgb<u8>> for Normal {
//     fn from(value: Rgb<u8>) -> Self {
//         Normal::new(
//             value[0] as f32 / 255.0 * 2.0 -
// 1.0,             value[1] as f32 / 255.0 * 2.0
// - 1.0,             value[2] as f32 / 255.0 *
// 2.0 - 1.0,         )
//     }
// }

#[cfg(test)]
mod tests {
	use super::{Albedo, Wrap};

	#[test]
	fn load_checker_texture() {
		let path = "../assets/texture/Checker-Texture.png";
		let texture = Albedo::load(path, Wrap::Clamp).unwrap();

		let (width, height) = texture.dimensions();
		assert_eq!(width, 1024);
		assert_eq!(height, 1024);
	}
}
