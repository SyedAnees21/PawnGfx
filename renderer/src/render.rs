use {
	crate::{
		buffer::Buffers,
		raster,
		shaders::{self, GlobalUniforms, LightUniforms, ScreenUniforms},
	},
	pcore::{
		error::PResult,
		math::{AffineMatrices, Matrix4},
	},
	pscene::global::Scene,
};

pub struct WinSize {
	pub width: u32,
	pub height: u32,
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
		let screen = self.uniforms();
		let aspect = screen.width / screen.height;

		self.reset_buffers();

		let (scale, position, rotation) = scene.object.get_transforms_props();

		let model = Matrix4::from_transforms(position, scale, rotation);
		let view = scene.camera.get_view_matrix();
		let projection =
			Matrix4::perspective_matrix(90.0_f64.to_radians(), aspect, 0.1, 100.0);

		let affine = AffineMatrices::from_mvp(model, view, projection);
		let light = LightUniforms {
			position: scene.light.position,
			direction: scene.light.direction(),
			ambient: scene.light.ambient,
		};

		let global_uniforms = GlobalUniforms {
			affine,
			screen,
			light,
			camera_pos: scene.camera.position,
			specular_strength: 0.4,
			shininess: 32.0,
		};

		let v_shader = shaders::Flat;
		let f_shader = shaders::Flat;

		raster::draw_call(
			&mut self.buffers,
			&global_uniforms,
			&scene.object.albedo,
			&scene.object.normal,
			scene.object.mesh.iter_triangles(),
			&v_shader,
			&f_shader,
		);

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
}
