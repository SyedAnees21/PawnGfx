use {
	crate::{
		buffer::Buffers,
		draw::DrawCall,
		raster,
		shaders::{BlinnPhong, Flat},
	},
	pcore::error::PResult,
	pscene::global::Scene,
};

#[derive(Clone, Copy)]
pub struct WinSize {
	pub width: u32,
	pub height: u32,
}

impl WinSize {
	pub fn aspect(&self) -> f64 {
		self.width as f64 / self.height as f64
	}
}

pub struct Renderer {
	win_size: WinSize,
	buffers: Buffers,
}

impl Renderer {
	pub fn new(win_width: u32, win_height: u32) -> Self {
		Renderer {
			win_size: WinSize {
				width: win_width,
				height: win_height,
			},
			buffers: Buffers::new(win_width, win_height),
		}
	}

	pub fn render<R>(&mut self, scene: &mut Scene, target: &mut R) -> PResult<()>
	where
		R: AsMut<[u8]> + ?Sized,
	{
		self.reset_buffers();

		let draw_call = DrawCall::submit_draw_call(scene, self.win_size);
		let blinn_phong = BlinnPhong;

		draw_call.execute(
			&mut self.buffers,
			&blinn_phong,
			raster::consume_draw_call,
		);

		target.as_mut().copy_from_slice(&self.buffers.f_buffer);
		Ok(())
	}

	pub fn reset_buffers(&mut self) {
		self.buffers.reset();
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.win_size.height = height;
		self.win_size.width = width;
		self.buffers.resize(width, height);
	}

	pub fn win_size(&self) -> &WinSize {
		&self.win_size
	}
}
