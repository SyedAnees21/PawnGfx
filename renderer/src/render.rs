use {
	crate::{
		buffer::Buffers,
		draw::DrawCall,
		raster,
		shaders::{BlinnPhong, ScreenUniforms},
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
		// let (scale, position, rotation) = scene.objects[0].get_transforms_props();

		// let model = Matrix4::from_transforms(position, scale, rotation);
		// let view = scene.camera.get_view_matrix();
		// let projection =
		// 	Matrix4::perspective_matrix(90.0_f64.to_radians(), aspect, 0.1, 100.0);

		// let affine = AffineMatrices::from_mvp(model, view, projection);
		// let light = LightUniforms {
		// 	position: scene.light.position,
		// 	direction: scene.light.direction(),
		// 	ambient: scene.light.ambient,
		// };

		// let global_uniforms = GlobalUniforms {
		// 	affine,
		// 	screen,
		// 	light,
		// 	camera_pos: scene.camera.position,
		// 	specular_strength: 0.6,
		// 	shininess: 64.0,
		// };

		// let v_shader = shaders::BlinnPhong;
		// let f_shader = shaders::BlinnPhong;

		// raster::draw_call(
		// 	&mut self.buffers,
		// 	&global_uniforms,
		// 	&scene.objects[0].albedo,
		// 	&scene.objects[0].normal,
		// 	scene.objects[0].mesh.iter_triangles(),
		// 	&v_shader,
		// 	&f_shader,
		// );

		target.as_mut().copy_from_slice(&self.buffers.f_buffer);
		Ok(())
	}

	pub fn uniforms(&self) -> ScreenUniforms {
		ScreenUniforms {
			width: self.win_size.width as f64,
			height: self.win_size.height as f64,
		}
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
