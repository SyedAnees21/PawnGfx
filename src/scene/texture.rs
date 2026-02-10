use std::path::Path;

use crate::{color::Color, error::PResult};

pub struct Texture {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            data: vec![],
        }
    }
}

impl Texture {
    pub fn new(w: usize, h: usize, data: Vec<Color>) -> Self {
        Self {
            width: w,
            height: h,
            data,
        }
    }

    pub fn from_file<P>(path: P) -> PResult<Self>
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

        Ok(Self {
            width: w as usize,
            height: h as usize,
            data,
        })
    }

    pub fn sample(&self, mut u: f64, mut v: f64) -> Color {
        // clamp UV to [0,1] for now
        u = u.clamp(0.0, 1.0);
        v = v.clamp(0.0, 1.0);

        // convert to pixel space
        let x = (u * (self.width as f64 - 1.0)) as usize;
        let y = (v * (self.height as f64 - 1.0)) as usize;

        self.data[y * self.width + x]
    }
}

#[cfg(test)]
mod tests {
    use crate::scene::Texture;

    #[test]
    fn load_checker_texture() {
        let path = "./assets/texture/Checker-Texture.png";
        let texture = Texture::from_file(path).unwrap();

        assert_eq!(texture.width, 1024);
        assert_eq!(texture.height, 1024);
        assert_eq!(texture.data.len(), 1024 * 1024);
    }
}
