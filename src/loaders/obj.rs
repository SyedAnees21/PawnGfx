use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{
    error::{FileError, PResult},
    geometry::{Indices, Mesh},
    math::{Vector2, Vector3},
};

pub fn load_obj<F>(path: F) -> PResult<Mesh>
where
    F: AsRef<Path>,
{
    let Some(ext) = path.as_ref().extension() else {
        return Err(FileError::Invalid.into());
    };

    if ext != "obj" {
        return Err(FileError::WrongFile("Wrong extension, expected .obj".to_string()).into());
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut uv = vec![];
    let mut vertices = vec![];
    let mut normals = vec![];
    let mut indices = Indices::default();

    for line in reader.lines() {
        let line = line?;
        let parts = line.trim().split_whitespace().collect::<Vec<_>>();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "v" => parse_vector3(&parts[1..], &mut vertices)?,
            "vn" => parse_vector3(&parts[1..], &mut normals)?,
            "vt" => parse_vector2(&parts[1..], &mut uv)?,
            "f" => parse_indices(&parts[1..], &mut indices)?,
            _ => {
                continue;
            }
        }
    }

    let mesh = Mesh::new(vertices, uv, indices, normals);
    Ok(mesh)
}

pub fn parse_vector3(parts: &[&str], vertices: &mut Vec<Vector3>) -> PResult<()> {
    let v = Vector3 {
        x: parts[0].parse()?,
        y: parts[1].parse()?,
        z: parts[2].parse()?,
    };

    vertices.push(v);
    Ok(())
}

pub fn parse_vector2(parts: &[&str], vertices: &mut Vec<Vector2>) -> PResult<()> {
    let v = Vector2 {
        x: parts[0].parse()?,
        y: parts[1].parse()?,
    };

    vertices.push(v);
    Ok(())
}

pub fn parse_indices(parts: &[&str], indices: &mut Indices) -> PResult<()> {
    let segment = parts[0];
    let seg_parts = segment.split('/').collect::<Vec<_>>();

    let parse_index = |part: &str| -> PResult<usize> {
        let index = part.parse::<usize>()?;
        Ok(index - 1)
    };

    let v = parse_index(seg_parts[0])?;
    indices.push_v_index(v);

    if seg_parts.len() > 1 && !seg_parts[1].is_empty() {
        let uv = parse_index(seg_parts[1])?;
        indices.push_uv_index(uv);
    }

    if seg_parts.len() > 2 && !seg_parts[2].is_empty() {
        let n = parse_index(seg_parts[2])?;
        indices.push_n_index(n);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::loaders::obj::load_obj;

    #[test]
    fn load_example_cube() {
        let file_path = "./assets/cube.obj";
        let mesh = load_obj(file_path).unwrap();

        assert!(
            mesh.indices.v.len() == mesh.indices.n.len()
                && mesh.indices.v.len() == mesh.indices.t.len()
        );

        assert!(mesh.uv.len() == 14);
        assert!(mesh.normals.len() == 6);
        assert!(mesh.vertices.len() == 8);
    }
}
