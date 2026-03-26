use {
	crate::{
		assets::registry::AssetRegistry,
		camera::Camera,
		light::Light,
		object::Object,
		texture::{AlbedoMap as Albedo, NormalMap, Wrap},
	},
	pcore::{
		geometry::AABB,
		math::{Matrix4, Vector3, Vector4},
	},
};

pub struct Scene {
	pub assets: AssetRegistry,
	pub objects: Vec<Object>,
	pub camera: Camera,
	pub light: Light,
}

impl Scene {
	pub fn center_aabb(&self) -> Vector3 {
		let mut min = Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
		let mut max =
			Vector3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
		let mut any = false;

		for object in &self.objects {
			let mesh_ref = self.assets.get_mesh(&object.model.mesh);

			let Some(mesh) = mesh_ref else { continue };
			let AABB {
				min: lmin,
				max: lmax,
			} = mesh.local_aabb();

			let model = Matrix4::from_transforms(
				object.transform.position,
				object.transform.scale,
				object.transform.rotation,
			);

			let corners = [
				Vector3::new(lmin.x, lmin.y, lmin.z),
				Vector3::new(lmin.x, lmin.y, lmax.z),
				Vector3::new(lmin.x, lmax.y, lmin.z),
				Vector3::new(lmin.x, lmax.y, lmax.z),
				Vector3::new(lmax.x, lmin.y, lmin.z),
				Vector3::new(lmax.x, lmin.y, lmax.z),
				Vector3::new(lmax.x, lmax.y, lmin.z),
				Vector3::new(lmax.x, lmax.y, lmax.z),
			];

			for c in corners {
				let w = (model * Vector4::from((c, 1.0))).xyz();
				if w.x < min.x {
					min.x = w.x;
				}
				if w.y < min.y {
					min.y = w.y;
				}
				if w.z < min.z {
					min.z = w.z;
				}

				if w.x > max.x {
					max.x = w.x;
				}
				if w.y > max.y {
					max.y = w.y;
				}
				if w.z > max.z {
					max.z = w.z;
				}
			}

			any = true;
		}

		if !any {
			return Vector3::ZERO;
		}

		(min + max) * 0.5
	}
}

impl Default for Scene {
	fn default() -> Self {
		let mut scene = Scene {
			camera: Camera::new(Vector3::new(0.0, 0.0, 5.0)),
			light: Light::default(),
			assets: AssetRegistry::default(),
			objects: Vec::new(),
		};

		// let camera = Camera::new(Vector3::new(0.0, 0.0, 5.0));

		let cube_mesh =
			crate::assets::load_mesh_file("./assets/meshes/cube-local.obj").unwrap();

		let albedo =
			Albedo::load("./assets/texture/Checker-Texture.png", Wrap::Mirror)
				.unwrap();

		let normal =
			NormalMap::load("./assets/texture/stones-normal.png", Wrap::Repeat)
				.unwrap();

		let mut object = Object::new(cube_mesh);

		object.set_albedo(albedo);
		object.set_normal_map(normal);

		scene.objects.push(object);
		// let light = Light::default();

		// Self {
		// 	assets: AssetRegistry::default(),
		// 	objects: vec![object],
		// 	camera,
		// 	light,
		// }

		scene
	}
}
