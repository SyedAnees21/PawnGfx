use pscene::color::Color;

const DEFAULT_BG_COLOR: u8 = 77;
const DEFAULT_DEPTH: f32 = f32::INFINITY;

pub type FrameBuffer = Vec<u8>;
pub type DepthBuffer = Vec<f32>;

#[derive(Default)]
pub struct Buffers {
	pub f_buffer: FrameBuffer,
	pub z_buffer: DepthBuffer,
}

impl Buffers {
	pub fn new(width: u32, height: u32) -> Self {
		let size = width * height;
		Self {
			f_buffer: vec![DEFAULT_BG_COLOR; (size * 4) as usize],
			z_buffer: vec![DEFAULT_DEPTH; size as usize],
		}
	}

	pub fn reset(&mut self) {
		self.f_buffer.fill(DEFAULT_BG_COLOR);
		self.z_buffer.fill(DEFAULT_DEPTH);
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		let size = width * height;
		self.f_buffer.resize((size * 4) as usize, DEFAULT_BG_COLOR);
		self.z_buffer.resize(size as usize, DEFAULT_DEPTH);
	}

	pub fn mut_buffers(&mut self) -> (&mut FrameBuffer, &mut DepthBuffer) {
		(&mut self.f_buffer, &mut self.z_buffer)
	}

	pub fn get_cursor(&mut self, offset: usize) -> Cursor {
		assert!(
			offset * 4 < self.f_buffer.len() && offset < self.z_buffer.len(),
			"Memory out of bounds in buffers, offset={}, zbuf={}, fbuf={}",
			offset,
			self.z_buffer.len(),
			self.f_buffer.len()
		);

		unsafe {
			Cursor {
				f_buffer: self.f_buffer.as_mut_ptr().add(offset * 4),
				z_buffer: self.z_buffer.as_mut_ptr().add(offset),
			}
		}
	}
}

pub type RawFrameBuffer = *mut u8;
pub type RawZBuffer = *mut f32;

pub struct Cursor {
	f_buffer: RawFrameBuffer,
	z_buffer: RawZBuffer,
}

impl Cursor {
	#[inline(always)]
	pub fn increment(&mut self, offset: usize) {
		unsafe {
			self.z_buffer = self.z_buffer.add(offset);
			self.f_buffer = self.f_buffer.add(offset * 4);
		}
	}

	#[inline(always)]
	pub fn get_depth(&self) -> f32 {
		unsafe { *self.z_buffer }
	}

	#[inline(always)]
	pub fn put_depth(&self, z: f32) {
		unsafe {
			*self.z_buffer = z;
		}
	}

	#[inline(always)]
	pub fn put_pixel(&self, color: Color) {
		unsafe {
			*(self.f_buffer as *mut u32) = u32::from_le_bytes(color.to_rgba8())
		}
	}

	#[inline(always)]
	pub fn step(&mut self) {
		unsafe {
			self.z_buffer = self.z_buffer.add(1);
			self.f_buffer = self.f_buffer.add(4);
		}
	}
}
