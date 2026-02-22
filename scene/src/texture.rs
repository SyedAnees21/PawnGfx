use std::path::Path;

use image::Rgb;

use crate::color::Color;
use pcore::{
    error::PResult,
    geometry::Normal,
    math::{self, Arithmetic},
};

pub enum Wrap {
    clamp,
    Repeat,
    Mirror,
}

pub struct Texture<T> {
    width: usize,
    height: usize,
    wrap: Wrap,
    data: Vec<T>,
}

impl<T> Default for Texture<T> {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            wrap: Wrap::clamp,
            data: vec![],
        }
    }
}

impl<T> Texture<T> {
    pub fn new(w: usize, h: usize, data: Vec<T>) -> Self {
        Self {
            width: w,
            height: h,
            wrap: Wrap::clamp,
            data,
        }
    }

    pub fn from_file<P>(path: P, wrap_mode: Wrap) -> PResult<Self>
    where
        P: AsRef<Path>,
        T: From<Rgb<u8>>,
    {
        let img = image::open(path)?.to_rgb8();
        let (w, h) = img.dimensions();

        let mut data = Vec::with_capacity((w * h) as usize);

        for pixel in img.pixels() {
            data.push(T::from(*pixel))
        }

        Ok(Self {
            width: w as usize,
            height: h as usize,
            wrap: wrap_mode,
            data,
        })
    }

    #[inline(always)]
    pub fn texel(&self, u: usize, v: usize) -> T
    where
        T: Copy,
    {
        self.data[v * self.width + u]
    }

    #[inline(always)]
    pub fn wrap_uv(&self, mut p: f64) -> f64 {
        match self.wrap {
            Wrap::clamp => p.clamp(0.0, 1.0),
            Wrap::Repeat => {
                if p.fract() < 0.0 {
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

    #[inline(always)]
    #[allow(unused)]
    pub fn sample(&self, mut u: f64, mut v: f64) -> T
    where
        T: Copy,
    {
        u = self.wrap_uv(u);
        v = self.wrap_uv(v);

        // Flipping image space
        v = 1.0 - v;

        // convert to pixel space
        let x = (u * (self.width as f64 - 1.0)) as usize;
        let y = (v * (self.height as f64 - 1.0)) as usize;

        // self.data[y * self.width + x]
        self.texel(x, y)
    }

    #[inline(always)]
    pub fn bi_sample(&self, mut u: f64, mut v: f64) -> T
    where
        T: Copy + Arithmetic,
    {
        u = self.wrap_uv(u);
        v = self.wrap_uv(v);

        // Flipping image space
        v = 1.0 - v;

        let x = u * (self.width as f64 - 1.0);
        let y = v * (self.height as f64 - 1.0);

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

// impl<T: From<Rgb<u8>>> AssetLoader for Texture<T> {
//     fn load_from_file<P>(path: P) -> PResult<Self>
//     where
//         P: AsRef<Path>,
//         Self: Sized,
//     {
//         let img = image::open(path)?.to_rgb8();
//         let (w, h) = img.dimensions();

//         let mut data = Vec::with_capacity((w * h) as usize);

//         for pixel in img.pixels() {
//             data.push(T::from(*pixel))
//         }

//         Ok(Self {
//             width: w as usize,
//             height: h as usize,
//             wrap: wrap_mode,
//             data,
//         })
//     }
// }

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

pub type NormalMap = Texture<Normal>;

// impl From<Rgb<u8>> for Normal {
//     fn from(value: Rgb<u8>) -> Self {
//         Normal::new(
//             value[0] as f64 / 255.0 * 2.0 - 1.0,
//             value[1] as f64 / 255.0 * 2.0 - 1.0,
//             value[2] as f64 / 255.0 * 2.0 - 1.0,
//         )
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
        let path = "./assets/texture/Checker-Texture.png";
        let texture = Texture::<Color>::from_file(path, Wrap::clamp).unwrap();

        assert_eq!(texture.width, 1024);
        assert_eq!(texture.height, 1024);
        assert_eq!(texture.data.len(), 1024 * 1024);
    }
}
