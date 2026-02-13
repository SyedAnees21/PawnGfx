use std::path::Path;

use crate::{color::Color, error::PResult, math};

pub enum Wrap {
    clamp,
    Repeat,
    Mirror,
}

#[derive(Clone)]
pub struct Mip {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Mip {
    pub fn new(width: usize, height: usize, data: Vec<Color>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn texel(&self, u: usize, v: usize) -> Color {
        self.data[v * self.width + u]
    }

    pub fn sample(&self, u: f64, v: f64) -> Color {
        let x = (u * (self.width as f64 - 1.0)) as usize;
        let y = (v * (self.height as f64 - 1.0)) as usize;

        self.texel(x, y)
    }

    pub fn bi_sample(&self, u: f64, v: f64) -> Color {
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

pub struct Texture {
    mipmap: Vec<Mip>,
    wrap: Wrap,
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            mipmap: vec![],
            wrap: Wrap::clamp,
        }
    }
}

impl Texture {
    pub fn new(w: usize, h: usize, data: Vec<Color>, wrap_mode: Wrap) -> Self {
        Self {
            mipmap: vec![Mip::new(w, h, data)],
            wrap: wrap_mode,
        }
    }

    pub fn from_file<P>(path: P, wrap_mode: Wrap) -> PResult<Self>
    where
        P: AsRef<Path>,
    {
        let img = image::open(path)?.to_rgb8();
        let (w, h) = img.dimensions();

        let mut data = Vec::with_capacity((w * h) as usize);

        for pixel in img.pixels() {
            data.push(Color::new_rgb(
                pixel[0] as f32 / 255.0,
                pixel[1] as f32 / 255.0,
                pixel[2] as f32 / 255.0,
            ));
        }

        let mut tex = Self::new(w as usize, h as usize, data, wrap_mode);
        tex.bake();

        Ok(tex)
    }

    pub fn bake(&mut self) {
        let mut base = &self.mipmap[0];

        while base.width > 1 || base.height > 1 {
            let new_w = (base.width / 2).max(1);
            let new_h = (base.height / 2).max(1);

            let mut new_data = vec![Color::BLACK; new_w * new_h];

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

                    new_data[y * new_w + x] = c0.add_raw(c1).add_raw(c2).add_raw(c3) * 0.25;
                }
            }

            let next = Mip::new(new_w, new_h, new_data);
            self.mipmap.push(next);
            base = self.mipmap.last().unwrap();
        }
    }

    pub fn get_level(&self, lod: usize) -> &Mip {
        let level = lod.min(self.mipmap.len() - 1);
        &self.mipmap[level]
    }

    pub fn wrap_uv(&self, mut p: f64) -> f64 {
        match self.wrap {
            Wrap::clamp => p.clamp(0.0, 1.0),
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

    pub fn sample(&self, mut u: f64, mut v: f64, lod: usize) -> Color {
        u = self.wrap_uv(u);
        v = self.wrap_uv(v);

        // Flipping image space
        v = 1.0 - v;

        self.get_level(lod).sample(u, v)
    }

    pub fn bi_sample(&self, mut u: f64, mut v: f64, lod: usize) -> Color {
        u = self.wrap_uv(u);
        v = self.wrap_uv(v);

        // Flipping image space
        v = 1.0 - v;

        self.get_level(lod).bi_sample(u, v)
    }

    pub fn tri_sample(&self, mut u: f64, mut v: f64, lod: f64) -> Color {
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

    pub fn max_lod(&self) -> f64 {
        if self.mipmap.is_empty() {
            0.0
        } else {
            (self.mipmap.len() - 1) as f64
        }
    }

    pub fn clamp_lod(&self, lod: f64) -> f64 {
        lod.clamp(0.0, self.max_lod())
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::scene::{Texture, Wrap};

//     #[test]
//     fn load_checker_texture() {
//         let path = "./assets/texture/Checker-Texture.png";
//         let texture = Texture::from_file(path, Wrap::clamp).unwrap();

//         assert_eq!(texture.width, 1024);
//         assert_eq!(texture.height, 1024);
//         assert_eq!(texture.data.len(), 1024 * 1024);
//     }
// }
