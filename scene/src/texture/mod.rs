use pcore::{geometry::UV, math::Arithmetic};
pub use {albedo::*, base::*, bump::*};

mod albedo;
pub mod base;
mod bump;

pub trait TextureSampler {
	type Out: Copy + Arithmetic;

	fn sample(&self, u: f32, v: f32, lod: f32) -> Self::Out;

	fn bi_sample(&self, u: f32, v: f32, lod: f32) -> Self::Out;

	fn tri_sample(&self, u: f32, v: f32, lod: f32) -> Self::Out {
		self.sample(u, v, lod)
	}
}

#[inline(always)]
pub fn unit_lod(duv_dx: UV, duv_dy: UV) -> f32 {
	let du_dx = duv_dx.x;
	let dv_dx = duv_dx.y;
	let du_dy = duv_dy.x;
	let dv_dy = duv_dy.y;

	// 4. Calculate squared lengths of the pixel footprints
	let len_x_sq = (du_dx * du_dx) + (dv_dx * dv_dx);
	let len_y_sq = (du_dy * du_dy) + (dv_dy * dv_dy);

	// 5. LOD = log2(max(len_x, len_y))
	// Which is the same as: 0.5 * log2(max(len_x_sq, len_y_sq))
	let rho_sq = len_x_sq.max(len_y_sq).max(1e-8);
	0.5 * rho_sq.log2()
}

#[inline(always)]
pub fn sized_lod(width: f32, height: f32, duv_dx: UV, duv_dy: UV) -> f32 {
	let du_dx_s = duv_dx.x * width;
	let dv_dx_s = duv_dx.y * height;
	let du_dy_s = duv_dy.x * width;
	let dv_dy_s = duv_dy.y * height;

	// 4. Calculate squared lengths of the pixel footprints
	let len_x_sq = (du_dx_s * du_dx_s) + (dv_dx_s * dv_dx_s);
	let len_y_sq = (du_dy_s * du_dy_s) + (dv_dy_s * dv_dy_s);

	// 5. LOD = log2(max(len_x, len_y))
	// Which is the same as: 0.5 * log2(max(len_x_sq, len_y_sq))
	let rho_sq = len_x_sq.max(len_y_sq).max(1e-8);
	0.5 * rho_sq.log2()
}

// pub fn from_file<P, C, A>(
// 	path: P,
// 	wrap_mode: Wrap,
// 	converter: C,
// 	averager: A,
// ) -> PResult<Self>
// where
// 	P: AsRef<Path>,
// 	T: Copy + Arithmetic + Default,
// 	C: Fn(Rgb<u8>) -> T,
// 	A: Fn(T, T, T, T) -> T,
// {
// 	let img = image::open(path)?.to_rgb8();
// 	let (w, h) = img.dimensions();

// 	let w = w as usize;
// 	let h = h as usize;

// 	let x_tiles = (w + TILE_SIZE - 1) / TILE_SIZE;
// 	let y_tiles = (h + TILE_SIZE - 1) / TILE_SIZE;

// 	let padded_w = x_tiles * TILE_SIZE;
// 	let padded_h = y_tiles * TILE_SIZE;

// 	let mut data = vec![T::default(); padded_w * padded_h];

// 	for y in 0..h {
// 		for x in 0..w {
// 			let pixel = img.get_pixel(x as u32, y as u32);
// 			let texel = converter(*pixel);

// 			// let tile_x = x / TILE_SIZE;
// 			// let tile_y = y / TILE_SIZE;

// 			// let local_x = x % TILE_SIZE;
// 			// let local_y = y % TILE_SIZE;

// 			// let tile_index = tile_y * x_tiles + tile_x;
// 			// let in_tile = local_y * TILE_SIZE + local_x;

// 			// let index = tile_index * (TILE_SIZE * TILE_SIZE) + in_tile;
// 			let index = Mip::<T>::index(x, y, x_tiles);
// 			data[index] = texel;
// 		}
// 	}

// 	let mut tex = Self::new(w as usize, h as usize, data, wrap_mode);
// 	tex.bake(averager);

// 	Ok(tex)
// }

// pub fn bake<A>(&mut self, averager: A)
// where
// 	T: Copy + Arithmetic + Default,
// 	A: Fn(T, T, T, T) -> T,
// {
// 	let mut level: usize = 0;

// 	loop {
// 		let base = &self.mipmap[level];

// 		if base.width <= 1 && base.height <= 1 {
// 			break;
// 		}

// 		let new_w = (base.width / 2).max(1);
// 		let new_h = (base.height / 2).max(1);

// 		let x_tiles = (new_w + TILE_SIZE - 1) / TILE_SIZE;
// 		let y_tiles = (new_h + TILE_SIZE - 1) / TILE_SIZE;

// 		let padded_w = x_tiles * TILE_SIZE;
// 		let padded_h = y_tiles * TILE_SIZE;

// 		let mut new_data = vec![T::default(); padded_w * padded_h];

// 		for y in 0..new_h {
// 			for x in 0..new_w {
// 				let x0 = (2 * x).min(base.width - 1);
// 				let x1 = (2 * x + 1).min(base.width - 1);
// 				let y0 = (2 * y).min(base.height - 1);
// 				let y1 = (2 * y + 1).min(base.height - 1);

// 				let c0 = base.unsafe_texel(x0, y0);
// 				let c1 = base.unsafe_texel(x1, y0);
// 				let c2 = base.unsafe_texel(x0, y1);
// 				let c3 = base.unsafe_texel(x1, y1);

// 				let avg_texel = averager(c0, c1, c2, c3);
// 				let index = Mip::<T>::index(x, y, x_tiles);

// 				new_data[index] = avg_texel;
// 			}
// 		}

// 		self.mipmap.push(Mip::new(new_w, new_h, new_data));
// 		level += 1;
// 	}
// }
