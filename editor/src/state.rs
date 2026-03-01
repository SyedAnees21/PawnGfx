use {
	pcore::geometry::Mesh,
	pscene::{
		assets,
		texture::{Albedo, NormalMap, Wrap},
	},
	std::{
		collections::HashMap,
		path::{Path, PathBuf},
	},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
	Select,
	Move,
	Rotate,
	Scale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeType {
	Cube,
	Sphere,
	Plane,
}

#[derive(Debug, Clone)]
pub struct Transform {
	pub position: [f32; 3],
	pub rotation: [f32; 3],
	pub scale: [f32; 3],
}

impl Default for Transform {
	fn default() -> Self {
		Self {
			position: [0.0, 0.0, 0.0],
			rotation: [0.0, 0.0, 0.0],
			scale: [1.0, 1.0, 1.0],
		}
	}
}

#[derive(Debug, Clone)]
pub struct SceneObject {
	pub id: usize,
	pub name: String,
	pub obj_type: String,
	pub transform: Transform,
	pub mesh: Option<String>,
	pub material: Option<String>,
	pub mesh_id: Option<AssetId>,
	pub albedo_id: Option<AssetId>,
	pub normal_id: Option<AssetId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetKind {
	Mesh,
	Albedo,
	Normal,
}

pub type AssetId = u32;

#[derive(Debug, Clone)]
pub struct AssetRecord {
	pub id: AssetId,
	pub name: String,
	pub path: PathBuf,
	pub kind: AssetKind,
}

#[derive(Default)]
pub struct AssetRegistry {
	next_id: AssetId,
	meshes: HashMap<AssetId, Mesh>,
	albedos: HashMap<AssetId, Albedo>,
	normals: HashMap<AssetId, NormalMap>,
	records: Vec<AssetRecord>,
	last_error: Option<String>,
}

impl AssetRegistry {
	pub fn records(&self) -> &[AssetRecord] {
		&self.records
	}

	pub fn last_error(&self) -> Option<&str> {
		self.last_error.as_deref()
	}

	pub fn record_name(&self, id: AssetId) -> Option<&str> {
		self
			.records
			.iter()
			.find(|r| r.id == id)
			.map(|r| r.name.as_str())
	}

	pub fn load_mesh(&mut self, path: PathBuf) -> Option<AssetId> {
		match assets::load_mesh_file(&path) {
			Ok(mesh) => Some(self.insert_mesh(path, mesh)),
			Err(err) => {
				self.last_error = Some(err.to_string());
				None
			}
		}
	}

	pub fn load_albedo(&mut self, path: PathBuf) -> Option<AssetId> {
		match Albedo::from_file(&path, Wrap::Repeat) {
			Ok(tex) => Some(self.insert_albedo(path, tex)),
			Err(err) => {
				self.last_error = Some(err.to_string());
				None
			}
		}
	}

	pub fn load_normal(&mut self, path: PathBuf) -> Option<AssetId> {
		match NormalMap::from_file(&path, Wrap::Repeat) {
			Ok(tex) => Some(self.insert_normal(path, tex)),
			Err(err) => {
				self.last_error = Some(err.to_string());
				None
			}
		}
	}

	fn insert_mesh(&mut self, path: PathBuf, mesh: Mesh) -> AssetId {
		let id = self.alloc_id();
		let name = file_stem(&path);
		self.meshes.insert(id, mesh);
		self.records.push(AssetRecord {
			id,
			name,
			path,
			kind: AssetKind::Mesh,
		});
		id
	}

	fn insert_albedo(&mut self, path: PathBuf, tex: Albedo) -> AssetId {
		let id = self.alloc_id();
		let name = file_stem(&path);
		self.albedos.insert(id, tex);
		self.records.push(AssetRecord {
			id,
			name,
			path,
			kind: AssetKind::Albedo,
		});
		id
	}

	fn insert_normal(&mut self, path: PathBuf, tex: NormalMap) -> AssetId {
		let id = self.alloc_id();
		let name = file_stem(&path);
		self.normals.insert(id, tex);
		self.records.push(AssetRecord {
			id,
			name,
			path,
			kind: AssetKind::Normal,
		});
		id
	}

	fn alloc_id(&mut self) -> AssetId {
		let id = self.next_id;
		self.next_id += 1;
		id
	}
}

fn file_stem(path: &Path) -> String {
	path
		.file_stem()
		.unwrap_or_default()
		.to_string_lossy()
		.to_string()
}

pub struct EditorState {
	pub scene_objects: Vec<SceneObject>,
	pub selected: Option<usize>,
	pub tool: Tool,
	pub assets: AssetRegistry,
	pub viewport: ViewportState,
}

pub struct ViewportState {
	pub width: usize,
	pub height: usize,
	pub framebuffer: Vec<u8>,
}

impl EditorState {
	pub fn new() -> Self {
		let mut state = Self {
			scene_objects: Vec::new(),
			selected: None,
			tool: Tool::Select,
			assets: AssetRegistry::default(),
			viewport: ViewportState {
				width: 800,
				height: 600,
				framebuffer: vec![0; 800 * 600 * 4],
			},
		};

		state.add_default_scene();
		state
	}

	fn add_default_scene(&mut self) {
		self.scene_objects.push(SceneObject {
			id: 0,
			name: "Main Camera".into(),
			obj_type: "Camera".into(),
			transform: Transform::default(),
			mesh: None,
			material: None,
			mesh_id: None,
			albedo_id: None,
			normal_id: None,
		});
		self.scene_objects.push(SceneObject {
			id: 1,
			name: "Directional Light".into(),
			obj_type: "Light".into(),
			transform: Transform::default(),
			mesh: None,
			material: None,
			mesh_id: None,
			albedo_id: None,
			normal_id: None,
		});
		self.scene_objects.push(SceneObject {
			id: 2,
			name: "Cube".into(),
			obj_type: "Mesh".into(),
			transform: Transform::default(),
			mesh: Some("Cube".into()),
			material: Some("Default".into()),
			mesh_id: None,
			albedo_id: None,
			normal_id: None,
		});
	}

	pub fn set_tool(&mut self, tool: Tool) {
		self.tool = tool;
	}

	pub fn select(&mut self, id: usize) {
		self.selected = Some(id);
	}

	pub fn selected_object_mut(&mut self) -> Option<&mut SceneObject> {
		let id = self.selected?;
		self.scene_objects.iter_mut().find(|o| o.id == id)
	}

	pub fn selected_object(&self) -> Option<&SceneObject> {
		let id = self.selected?;
		self.scene_objects.iter().find(|o| o.id == id)
	}

	pub fn add_shape(&mut self, shape: ShapeType) {
		let id = self.scene_objects.len();
		let (name, obj_type) = match shape {
			ShapeType::Cube => ("Cube", "Mesh"),
			ShapeType::Sphere => ("Sphere", "Mesh"),
			ShapeType::Plane => ("Plane", "Mesh"),
		};

		self.scene_objects.push(SceneObject {
			id,
			name: name.into(),
			obj_type: obj_type.into(),
			transform: Transform::default(),
			mesh: Some(name.into()),
			material: Some("Default".into()),
			mesh_id: None,
			albedo_id: None,
			normal_id: None,
		});
		self.selected = Some(id);
	}

	pub fn assign_mesh_to_selected(&mut self, asset_id: AssetId) {
		let name = self
			.assets
			.record_name(asset_id)
			.unwrap_or("Mesh")
			.to_string();
		if let Some(obj) = self.selected_object_mut() {
			obj.mesh_id = Some(asset_id);
			obj.mesh = Some(name);
		}
	}

	pub fn assign_albedo_to_selected(&mut self, asset_id: AssetId) {
		let name = self
			.assets
			.record_name(asset_id)
			.unwrap_or("Albedo")
			.to_string();
		if let Some(obj) = self.selected_object_mut() {
			obj.albedo_id = Some(asset_id);
			obj.material = Some(name);
		}
	}

	pub fn assign_normal_to_selected(&mut self, asset_id: AssetId) {
		if let Some(obj) = self.selected_object_mut() {
			obj.normal_id = Some(asset_id);
		}
	}
}
