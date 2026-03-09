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

#[derive(Clone)]
pub struct Mip<T> {
	width: usize,
	height: usize,
	data: Vec<T>,
}

impl<T> Mip<T> {
	pub fn new(width: usize, height: usize, data: Vec<T>) -> Self {
		Self {
			width,
			height,
			data,
		}
	}

	pub fn texel(&self, u: usize, v: usize) -> T
	where
		T: Copy,
	{
		self.data[v * self.width + u]
	}

	pub fn sample(&self, u: f32, v: f32) -> T
	where
		T: Copy,
	{
		let x = (u * (self.width as f32 - 1.0)) as usize;
		let y = (v * (self.height as f32 - 1.0)) as usize;

		self.texel(x, y)
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

		let c00 = self.texel(x0, y0);
		let c10 = self.texel(x1, y0);
		let c01 = self.texel(x0, y1);
		let c11 = self.texel(x1, y1);

		math::bi_lerp(c00, c01, c10, c11, tx, ty)
	}
}

pub enum Wrap {
	Clamp,
	Repeat,
	Mirror,
}

pub struct Texture<T> {
	mipmap: Vec<Mip<T>>,
	wrap: Wrap,
}

impl<T> Default for Texture<T> {
	fn default() -> Self {
		Self {
			mipmap: vec![],
			wrap: Wrap::Clamp,
		}
	}
}

impl<T> Texture<T> {
	pub fn new(w: usize, h: usize, data: Vec<T>, wrap_mode: Wrap) -> Self {
		Self {
			mipmap: vec![Mip::new(w, h, data)],
			wrap: wrap_mode,
		}
	}

	pub fn from_file<P>(path: P, wrap_mode: Wrap) -> PResult<Self>
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
		tex.bake();

		Ok(tex)
	}

	pub fn bake(&mut self)
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

					let c0 = base.texel(x0, y0);
					let c1 = base.texel(x1, y0);
					let c2 = base.texel(x0, y1);
					let c3 = base.texel(x1, y1);

					new_data[y * new_w + x] = (c0 + c1 + c2 + c3) * 0.25;
					// c0.add_raw(c1).add_raw(c2).add_raw(c3) * 0.25;
				}
			}

			let next = Mip::new(new_w, new_h, new_data);
			self.mipmap.push(next);
			base = self.mipmap.last().unwrap();
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

pub type Albedo = Texture<Color>;

impl From<Rgb<u8>> for Color {
	fn from(value: Rgb<u8>) -> Self {
		Color::new_rgb(
			value[0] as f32 / 255.0,
			value[1] as f32 / 255.0,
			value[2] as f32 / 255.0,
		)
	}
}

impl Albedo {
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

pub type NormalMap = Texture<Normal>;

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
	use crate::{
		color::Color,
		texture::{Texture, Wrap},
	};

	#[test]
	fn load_checker_texture() {
		let path = "../assets/texture/Checker-Texture.png";
		let texture = Texture::<Color>::from_file(path, Wrap::Clamp).unwrap();

		let (width, height) = texture.dimensions();
		assert_eq!(width, 1024);
		assert_eq!(height, 1024);
	}
}
