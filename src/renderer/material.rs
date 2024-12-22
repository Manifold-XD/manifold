use super::shader::ShaderType;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Material {
    pub name: String,
    pub shader_type: ShaderType,
    pub diffuse_texture_id: u32,
}

pub struct MaterialStore {
    pub default_material: Material,
    materials: HashMap<u32, Material>,
    next_id: u32,
}

impl MaterialStore {
    pub fn new() -> Self {
        let default_material = Material {
            name: ShaderType::Basic.to_string(),
            shader_type: ShaderType::Basic,
            diffuse_texture_id: 0,
        };
        let basic_material = Material {
            name: ShaderType::Basic.to_string(),
            shader_type: ShaderType::Basic,
            diffuse_texture_id: 0,
        };
        let grid_material = Material {
            name: ShaderType::Grid.to_string(),
            shader_type: ShaderType::Grid,
            diffuse_texture_id: 0,
        };

        let materials: HashMap<u32, Material> = vec![(0, basic_material), (1, grid_material)]
            .into_iter()
            .collect();

        let next_id = materials.len() as u32 + 1;

        Self {
            default_material,
            materials,
            next_id,
        }
    }

    pub fn add_material(&mut self, material: Material) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.materials.insert(id, material);
        id
    }

    pub fn get_material(&self, id: u32) -> &Material {
        self.materials.get(&id).unwrap_or(&self.default_material)
    }
}
