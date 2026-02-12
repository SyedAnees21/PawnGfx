use std::path::Path;

use crate::{color::Color, error::PResult, math};

pub enum Wrap {
    clamp,
    Repeat,
    Mirror,
}

pub struct Mip {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Mip {
    pub fn new(width: usize, height: usize, data: Vec<Color>) -> Self {
        Self { width, height, data }
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
            wrap: Wrap::clamp,
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

        Ok(Self::new(w as usize, h as usize,  data, wrap_mode))
    }

    // pub fn bake(&mut self) {

    // }

    pub fn texel(&self, u: usize, v: usize) -> Color {
        self.data[v * self.width + u]
    }

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

    pub fn sample(&self, mut u: f64, mut v: f64) -> Color {
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

    pub fn bi_sample(&self, mut u: f64, mut v: f64) -> Color {
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



#[cfg(test)]
mod tests {
    use crate::scene::{Texture, Wrap};

    #[test]
    fn load_checker_texture() {
        let path = "./assets/texture/Checker-Texture.png";
        let texture = Texture::from_file(path, Wrap::clamp).unwrap();

        assert_eq!(texture.width, 1024);
        assert_eq!(texture.height, 1024);
        assert_eq!(texture.data.len(), 1024 * 1024);
    }
}
