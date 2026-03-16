use std::path::Path;

use image::Rgb;
use pcore::{error::PResult, geometry::Normal, math};

use crate::texture::{Texture, TextureMap, TextureSampler, Wrap};

pub type NormalMap = TextureMap<Normal>;
pub type TNormal = Texture<Normal, NormalMap>;

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

impl TextureSampler for NormalMap {
	type Out = Normal;

	#[inline(always)]
	fn sample(&self, u: f32, v: f32, lod: f32) -> Self::Out {
		let lod = self.clamp_lod(lod);
		let mip = self.unsafe_get_level(lod as usize);

		let u = (u * mip.width as f32 - 0.5) as usize;
		let v = (v * mip.height as f32 - 0.5) as usize;

		let normal = mip.unsafe_texel(u, v);
		normal
	}

	#[inline(always)]
	fn bi_sample(&self, u: f32, v: f32, lod: f32) -> Self::Out {
		let u = self.wrap_uv(u);
		let mut v = self.wrap_uv(v);

		v = 1.0 - v;

		let lod = self.clamp_lod(lod);
		let mip = self.unsafe_get_level(lod as usize);

		let x = u * (mip.width as f32 - 1.0);
		let y = v * (mip.height as f32 - 1.0);

		let x0 = x.floor() as usize;
		let y0 = y.floor() as usize;
		let x1 = (x0 + 1).min(mip.width - 1);
		let y1 = (y0 + 1).min(mip.height - 1);

		let tx = x - x0 as f32;
		let ty = y - y0 as f32;

		let c00 = mip.unsafe_texel(x0, y0);
		let c10 = mip.unsafe_texel(x1, y0);
		let c01 = mip.unsafe_texel(x0, y1);
		let c11 = mip.unsafe_texel(x1, y1);

		math::bi_lerp(c00, c01, c10, c11, tx, ty).normalize()
	}
}

// pub type NormalMap = TextureMap<Normal32>;

// impl NormalMap {
// 	pub fn load<P>(path: P, wrap: Wrap) -> PResult<Self>
// 	where
// 		P: AsRef<Path>,
// 	{
// 		let converter = |p: Rgb<u8>| -> Normal32 {
// 			Normal32::new(
// 				p[0] as f32 / 255.0 * 2.0 - 1.0,
// 				p[1] as f32 / 255.0 * 2.0 - 1.0,
// 			)
// 		};

// 		let averager =
// 			|c0: Normal32, c1: Normal32, c2: Normal32, c3: Normal32| -> Normal32 {
// 				(c0 + c1 + c2 + c3) * 0.25
// 			};

// 		TextureMap::<Normal32>::from_file(path, wrap, converter, averager)
// 	}
// }

// impl TextureSampler for NormalMap {
// 	type Out = Normal;

// 	fn wrap_kind(&self) -> super::Wrap {
// 		self.wrap
// 	}

// 	#[inline(always)]
// 	fn sample(&self, u: f32, v: f32, lod: f32) -> Self::Out {
// 		let lod = self.clamp_lod(lod);
// 		let mip = self.get_level(lod as usize);

// 		let u = (u * (mip.width as f32 - 1.0)) as usize;
// 		let v = (v * (mip.height as f32 - 1.0)) as usize;

// 		let normal = mip.unsafe_texel(u, v);
// 		normal.unpack().normalize()
// 	}

//     #[inline(always)]
// 	fn bi_sample(&self, u: f32, v: f32, lod: f32) -> Self::Out {
//         let u = self.wrap_uv(u);
//         let mut v = self.wrap_uv(v);

// 		v = 1.0 - v;

// 		let lod = self.clamp_lod(lod);
// 		let mip = self.get_level(lod as usize);

//         let x = u * (mip.width as f32 - 1.0);
// 		let y = v * (mip.height as f32 - 1.0);

// 		let x0 = x as usize;
// 		let y0 = y as usize;
// 		let x1 = (x0 + 1).min(mip.width - 1);
// 		let y1 = (y0 + 1).min(mip.height - 1);

// 		let tx = (x.fract() * 256.0) as u32;
// 		let ty = (y.fract() * 256.0) as u32;

// 		let c00 = mip.unsafe_texel(x0, y0);
// 		let c10 = mip.unsafe_texel(x1, y0);
// 		let c01 = mip.unsafe_texel(x0, y1);
// 		let c11 = mip.unsafe_texel(x1, y1);

// 		Normal32::bi_lerp(c00, c10, c01, c11, tx, ty).unpack().normalize()
// 	}
// }
