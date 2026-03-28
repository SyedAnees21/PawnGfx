use {
	crate::{buffer::Buffers, draw::DrawCall, raster, shaders::{BlinnPhong, DepthOnly}},
	pcore::{
		error::PResult,
		math::{Matrix4, Vector3, Vector4},
	},
	pscene::global::Scene,
};

const SHADOW_MAP_SIZE: u32 = 1024;
const SHADOW_MARGIN: f32 = 1.0;
const SHADOW_MIN_DEPTH: f32 = 0.1;

#[derive(Clone, Copy)]
pub struct WinSize {
	pub width: u32,
	pub height: u32,
}

impl WinSize {
	pub fn aspect(&self) -> f32 {
		self.width as f32 / self.height as f32
	}
}

pub struct Renderer {
	win_size: WinSize,
	buffers: Buffers,
	shadow_buffers: Buffers,
}

impl Renderer {
	pub fn new(win_width: u32, win_height: u32) -> Self {
		Renderer {
			win_size: WinSize {
				width: win_width,
				height: win_height,
			},
			buffers: Buffers::new(win_width, win_height),
			shadow_buffers: Buffers::new(SHADOW_MAP_SIZE, SHADOW_MAP_SIZE),
		}
	}

	pub fn render<R>(&mut self, scene: &mut Scene, target: &mut R) -> PResult<()>
	where
		R: AsMut<[u8]> + ?Sized,
	{
		self.reset_buffers();

		let (light_view, light_proj) = build_light_matrices(scene);
		let light_vp = light_proj * light_view;

		// Shadow pass (depth-only)
		self.shadow_buffers.reset();
		let mut shadow_draw = DrawCall::submit_draw_call(
			scene,
			WinSize {
				width: SHADOW_MAP_SIZE,
				height: SHADOW_MAP_SIZE,
			},
		);
		{
			let u = shadow_draw.uniforms_mut();
			u.m_view = light_view;
			u.m_projection = light_proj;
			u.m_view_projection = light_vp;
		}
		let depth_only = DepthOnly;
		shadow_draw.execute(
			&mut self.shadow_buffers,
			&depth_only,
			raster::consume_draw_call,
		);

		// Main pass
		let mut draw_call = DrawCall::submit_draw_call(scene, self.win_size);
		{
			let u = draw_call.uniforms_mut();
			u.shadow.enabled = true;
			u.shadow.light_vp = light_vp;
			u.shadow.map_size = SHADOW_MAP_SIZE;
			u.shadow.bias = 0.005;
			u.shadow.strength = 0.02;
			u.shadow.depth_ptr = self.shadow_buffers.depth_ptr();
		}
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

fn build_light_matrices(scene: &Scene) -> (Matrix4, Matrix4) {
	let light_dir = scene.light.direction().normalize();
	let aabb = scene.aabb_world();

	let (center, radius) = if let Some(aabb) = aabb {
		let c = (aabb.min + aabb.max) * 0.5;
		let e = aabb.max - aabb.min;
		let r = (e.magnitude() * 0.5).max(1.0);
		(c, r)
	} else {
		(Vector3::ZERO, 5.0)
	};

	let light_pos = center + light_dir * (radius * 2.0);
	let view = look_at(light_pos, center, Vector3::UNIT_Y);

	if let Some(aabb) = aabb {
		let corners = [
			Vector3::new(aabb.min.x, aabb.min.y, aabb.min.z),
			Vector3::new(aabb.min.x, aabb.min.y, aabb.max.z),
			Vector3::new(aabb.min.x, aabb.max.y, aabb.min.z),
			Vector3::new(aabb.min.x, aabb.max.y, aabb.max.z),
			Vector3::new(aabb.max.x, aabb.min.y, aabb.min.z),
			Vector3::new(aabb.max.x, aabb.min.y, aabb.max.z),
			Vector3::new(aabb.max.x, aabb.max.y, aabb.min.z),
			Vector3::new(aabb.max.x, aabb.max.y, aabb.max.z),
		];

		let mut min = Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
		let mut max =
			Vector3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

		for c in corners {
			let v = (view * Vector4::from((c, 1.0))).xyz();
			if v.x < min.x {
				min.x = v.x;
			}
			if v.y < min.y {
				min.y = v.y;
			}
			if v.z < min.z {
				min.z = v.z;
			}

			if v.x > max.x {
				max.x = v.x;
			}
			if v.y > max.y {
				max.y = v.y;
			}
			if v.z > max.z {
				max.z = v.z;
			}
		}

		let left = min.x - SHADOW_MARGIN;
		let right = max.x + SHADOW_MARGIN;
		let bottom = min.y - SHADOW_MARGIN;
		let top = max.y + SHADOW_MARGIN;
		let near = (-max.z).max(SHADOW_MIN_DEPTH);
		let far = (-min.z + SHADOW_MARGIN).max(near + 0.1);

		let proj =
			Matrix4::orthographic_matrix(left, right, bottom, top, near, far);
		(view, proj)
	} else {
		let extent = radius + SHADOW_MARGIN;
		let proj = Matrix4::orthographic_matrix(
			-extent,
			extent,
			-extent,
			extent,
			SHADOW_MIN_DEPTH,
			extent * 4.0,
		);
		(view, proj)
	}
}

fn look_at(eye: Vector3, target: Vector3, up: Vector3) -> Matrix4 {
	let f = (target - eye).normalize();
	let r = f.cross(&up).normalize();
	let u = r.cross(&f).normalize();

	Matrix4 {
		data: [
			[r.x, r.y, r.z, -r.dot(&eye)],
			[u.x, u.y, u.z, -u.dot(&eye)],
			[-f.x, -f.y, -f.z, f.dot(&eye)],
			[0.0, 0.0, 0.0, 1.0],
		],
	}
}
