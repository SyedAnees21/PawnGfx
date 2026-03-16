use std::path::Path;

use image::Rgb;
use pcore::{color::Color, error::PResult, math};

use crate::texture::{Texture, TextureMap, TextureSampler, Wrap};

pub type AlbedoMap = TextureMap<Color>;
pub type TAlbedo = Texture<Color, AlbedoMap>;

impl AlbedoMap {
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
}

impl TextureSampler for AlbedoMap {
	type Out = Color;

	#[inline(always)]
	fn sample(&self, u: f32, v: f32, lod: f32) -> Self::Out {
		let lod = self.clamp_lod(lod);
		let mip = self.unsafe_get_level(lod as usize);

		let u = (u * mip.width as f32 - 0.5) as usize;
		let v = (v * mip.height as f32 - 0.5) as usize;

		let color = mip.unsafe_texel(u, v);
		color
	}

	#[inline(always)]
	fn bi_sample(&self, u: f32, v: f32, lod: f32) -> Self::Out {
		let lod = self.clamp_lod(lod);
		let mip = self.unsafe_get_level(lod as usize);

		let u = self.wrap_uv(u);
		let mut v = self.wrap_uv(v);

		v = 1.0 - v;

		let x = u * (mip.width as f32 - 1.0);
		let y = v * (mip.height as f32 - 1.0);

		let x0 = x.floor() as usize;
		let y0 = y.floor() as usize;
		let x1 = (x0 + 1).min(mip.width - 1);
		let y1 = (y0 + 1).min(mip.height - 1);

		let tx = x.fract();
		let ty = y.fract();

		let c00 = mip.unsafe_texel(x0, y0);
		let c10 = mip.unsafe_texel(x1, y0);
		let c01 = mip.unsafe_texel(x0, y1);
		let c11 = mip.unsafe_texel(x1, y1);

		math::bi_lerp(c00, c01, c10, c11, tx, ty)
	}

	#[inline(always)]
	fn tri_sample(&self, u: f32, v: f32, lod: f32) -> Self::Out {
		let u = self.wrap_uv(u);
		let mut v = self.wrap_uv(v);

		v = 1.0 - v;

		let lod = self.clamp_lod(lod);
		let lod_0 = lod.floor() as usize;
		let lod_1 = (lod_0 + 1).min(self.mipmap.len() - 1);

		let t = lod.fract();

		// Reason for redefining the bi-sampling here without using the
		// existing bi-sampler is to avoid the redundant maths and unapcking
		// for the sake of some performance.
		let bi_sample = |u: f32, v: f32, lod: usize| -> Color {
			let mip = self.unsafe_get_level(lod);

			let x = u * (mip.width as f32 - 1.0);
			let y = v * (mip.height as f32 - 1.0);

			let x0 = x.floor() as usize;
			let y0 = y.floor() as usize;
			let x1 = (x0 + 1).min(mip.width - 1);
			let y1 = (y0 + 1).min(mip.height - 1);

			let tx = x.fract();
			let ty = y.fract();

			let c00 = mip.unsafe_texel(x0, y0);
			let c10 = mip.unsafe_texel(x1, y0);
			let c01 = mip.unsafe_texel(x0, y1);
			let c11 = mip.unsafe_texel(x1, y1);

			math::bi_lerp(c00, c01, c10, c11, tx, ty)
		};

		let c_0 = bi_sample(u, v, lod_0);
		let c_1 = bi_sample(u, v, lod_1);

		math::lerp(c_0, c_1, t)
	}
}

// pub type AlbedoMap = TextureMap<Color32>;
// pub type AlbedoTexture = Texture<Color, AlbedoMap>;

// impl AlbedoMap {
// 	pub fn load<P>(path: P, wrap: Wrap) -> PResult<Self>
// 	where
// 		P: AsRef<Path>,
// 	{
// 		let converter = |p: Rgb<u8>| -> Color32 {
// 			Color32::new(p[0] as u32, p[1] as u32, p[2] as u32, 255)
// 			// Color::new_rgb(
// 			// 	p[0] as f32 / 255.0,
// 			// 	p[1] as f32 / 255.0,
// 			// 	p[2] as f32 / 255.0,
// 			// )
// 		};

// 		// let averager =
// 		// 	|c0: Color32, c1: Color32, c2: Color32, c3: Color32| -> Color32 {
// 		// 		(c0 + c1 + c2 + c3) * 0.25
// 		// 	};

// 		let averager = |c0: Color32,
// 		                c1: Color32,
// 		                c2: Color32,
// 		                c3: Color32|
// 		 -> Color32 {
// 			let a0 = c0.0 as u64;
// 			let a1 = c1.0 as u64;
// 			let a2 = c2.0 as u64;
// 			let a3 = c3.0 as u64;

// 			const R_MSK: u64 = 0xFF;
// 			const G_MSK: u64 = 0xFF00;
// 			const B_MSK: u64 = 0xFF0000;
// 			const A_MSK: u64 = 0xFF000000;
// 			// Sum each channel individually using u64 to prevent overflow
// 			// Masking: R=0xFF, G=0xFF00, B=0xFF0000, A=0xFF000000
// 			let r = ((a0 & R_MSK) + (a1 & R_MSK) + (a2 & R_MSK) + (a3 & R_MSK)) >> 2;
// 			let g = ((a0 & G_MSK) + (a1 & G_MSK) + (a2 & G_MSK) + (a3 & G_MSK)) >> 2;
// 			let b = ((a0 & B_MSK) + (a1 & B_MSK) + (a2 & B_MSK) + (a3 & B_MSK)) >> 2;
// 			let a = ((a0 & A_MSK) + (a1 & A_MSK) + (a2 & A_MSK) + (a3 & A_MSK)) >> 2;
// 			// let r = ((a0 & 0xFF) + (a1 & 0xFF) + (a2 & 0xFF) + (a3 & 0xFF)) >> 2;
// 			// let g = (((a0 >> 8) & 0xFF) + ((a1 >> 8) & 0xFF) + ((a2 >> 8) & 0xFF) + ((a3 >> 8) & 0xFF)) >> 2;
// 			// let b = (((a0 >> 16) & 0xFF) + ((a1 >> 16) & 0xFF) + ((a2 >> 16) & 0xFF) + ((a3 >> 16) & 0xFF)) >> 2;
// 			// let a = (((a0 >> 24) & 0xFF) + ((a1 >> 24) & 0xFF) + ((a2 >> 24) & 0xFF) + ((a3 >> 24) & 0xFF)) >> 2;

// 			Color32(((r & R_MSK) | (g & G_MSK) | (b & B_MSK) | (a & A_MSK)) as u32)
// 			// Color32((r | (g << 8) | (b << 16) | (a << 24)) as u32)
// 		};

// 		TextureMap::<Color32>::from_file(path, wrap, converter, averager)
// 	}
// }

// impl TextureSampler for AlbedoMap {
// 	type Out = Color;

// 	fn wrap_kind(&self) -> super::Wrap {
// 		self.wrap
// 	}

// 	#[inline(always)]
// 	fn sample(&self, u: f32, v: f32, lod: f32) -> Self::Out {
// 		let lod = self.clamp_lod(lod);
// 		let mip = self.get_level(lod as usize);

// 		let u = (u * (mip.width as f32 - 1.0)) as usize;
// 		let v = (v * (mip.height as f32 - 1.0)) as usize;

// 		let color = mip.unsafe_texel(u, v);
// 		color.unpack()
// 	}

// 	#[inline(always)]
// 	fn bi_sample(&self, u: f32, v: f32, lod: f32) -> Self::Out {
// 		let lod = self.clamp_lod(lod);
// 		let mip = self.unsafe_get_level(lod as usize);

// 		let u = self.wrap_uv(u);
// 		let mut v = self.wrap_uv(v);

// 		v = 1.0 - v;

// 		let x = u * (mip.width as f32 - 1.0);
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

// 		Color32::bi_lerp(c00, c10, c01, c11, tx, ty).unpack()
// 	}

// 	#[inline(always)]
// 	fn tri_sample(&self, u: f32, v: f32, lod: f32) -> Self::Out {
// 		let u = self.wrap_uv(u);
// 		let mut v = self.wrap_uv(v);

// 		v = 1.0 - v;

// 		// Forced LOD
// 		// let lod = 2 as f32;
// 		let lod = self.clamp_lod(lod);
// 		let lod_0 = lod as usize;
// 		let lod_1 = (lod_0 + 1).min(self.mipmap.len() - 1);

// 		let t = (lod.fract() * 256.0) as u32;

// 		// Reason for redefining the bi-sampling here without using the
// 		// existing bi-sampler is to avoid the redundant maths and unapcking
// 		// for the sake of some performance.
// 		let bi_sample = |u: f32, v: f32, lod: usize| -> Color32 {
// 			let mip = self.unsafe_get_level(lod);

// 			let x = u * (mip.width as f32 - 1.0);
// 			let y = v * (mip.height as f32 - 1.0);

// 			let x0 = x as usize;
// 			let y0 = y as usize;
// 			let x1 = (x0 + 1).min(mip.width - 1);
// 			let y1 = (y0 + 1).min(mip.height - 1);

// 			let tx = (x.fract() * 256.0) as u32;
// 			let ty = (y.fract() * 256.0) as u32;

// 			let c00 = mip.unsafe_texel(x0, y0);
// 			let c10 = mip.unsafe_texel(x1, y0);
// 			let c01 = mip.unsafe_texel(x0, y1);
// 			let c11 = mip.unsafe_texel(x1, y1);

// 			Color32::bi_lerp(c00, c01, c10, c11, tx, ty)
// 		};

// 		let c_0 = bi_sample(u, v, lod_0);
// 		let c_1 = bi_sample(u, v, lod_1);

// 		Color32::lerp(c_0, c_1, t).unpack()
// 	}
// }
