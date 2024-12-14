use std::path::Path;

use cgmath::Rotation3;

use super::{
    context::Context,
    model::{Material, Mesh, Model},
    texture::TexturePool,
    util::resources,
};

#[allow(dead_code)]
pub struct Object {
    model: Model,
    position: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,
}
pub trait DrawObject<'a> {
    fn draw_mesh (
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
    );
    fn draw_object(&mut self, object: &'a Object, camera_bind_group: &'a wgpu::BindGroup);
}

impl<'a, 'b> DrawObject<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(
        &mut self,
        mesh: &'b Mesh,
        material: &'b Material,
        camera_bind_group: &'b wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, &camera_bind_group, &[]);
        self.set_bind_group(1, &material.bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, 0..1);
    }

    fn draw_object(&mut self, object: &'a Object, camera_bind_group: &'a wgpu::BindGroup) {
        for mesh in &object.model.meshes {
            let material = &object.model.materials[mesh.material];
            self.draw_mesh(mesh, material, camera_bind_group);
        }
    }
}

pub struct ObjectManager {
    actors: Vec<Object>,            // User-defined objects
    immutable_objects: [Object; 1], // Static objects
}

#[allow(dead_code)]
impl<'a> ObjectManager {
    pub async fn new(context: &'a Context<'a>, texture_pool: &'a TexturePool) -> Self {
        let obj_path = Path::new("models/cube.obj").to_path_buf();
        let obj_model = resources::load_model(
            &obj_path,
            &context.device,
            &context.queue,
            &texture_pool.bind_group_layout,
        )
        .await
        .unwrap();

        Self {
            actors: Vec::new(),
            immutable_objects: [Object {
                model: obj_model,
                position: cgmath::vec3(0.0, 0.0, 0.0),
                rotation: cgmath::Quaternion::from_axis_angle(
                    cgmath::Vector3::unit_y(),
                    cgmath::Deg(0.0),
                ),
            }],
        }
    }

    pub fn add_actor(&mut self, object: Object) {
        self.actors.push(object);
    }

    pub fn remove_actor(&mut self, index: usize) {
        self.actors.remove(index);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Object> {
        self.immutable_objects.iter().chain(self.actors.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Object> {
        self.immutable_objects
            .iter_mut()
            .chain(self.actors.iter_mut())
    }
}
