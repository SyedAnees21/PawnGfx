use {
	crate::{
		assets::registry::AssetRegistry, camera::Camera, light::Light, object::Object, texture::{Albedo, NormalMap, Wrap}
	},
	pcore::math::Vector3,
};

pub struct Scene {
	pub assets: AssetRegistry,
	pub objects: Vec<Object>,
	pub camera: Camera,
	pub light: Light,
}

impl Default for Scene {
	fn default() -> Self {
		let mut scene = Scene {
			camera: Camera::new(Vector3::new(0.0, 0.0, 5.0)),
			light: Light::default(),
			assets: AssetRegistry::default(),
			objects: Vec::new()
		};

		// let camera = Camera::new(Vector3::new(0.0, 0.0, 5.0));

		let cube_mesh =
			crate::assets::load_mesh_file("./assets/meshes/cube-local.obj").unwrap();

		let albedo =
			Albedo::from_file("./assets/texture/Checker-Texture.png", Wrap::Mirror)
				.unwrap();

		let normal =
			NormalMap::from_file("./assets/texture/stones-normal.png", Wrap::Repeat)
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
