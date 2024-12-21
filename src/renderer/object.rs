use std::path::Path;

use cgmath::Rotation3;
use wgpu::util::DeviceExt;

use super::{
    context::Context,
    material::MaterialStore,
    model::{load_model, Mesh, Model},
    texture::TextureStore,
};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformUniform {
    matrix: [[f32; 4]; 4],
}

#[allow(dead_code)]
pub struct Object {
    model: Model,
    position: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl Object {
    pub fn new(
        model: Model,
        position: cgmath::Vector3<f32>,
        rotation: cgmath::Quaternion<f32>,
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let uniform = TransformUniform {
            matrix: (cgmath::Matrix4::from_translation(position) * cgmath::Matrix4::from(rotation))
                .into(),
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Object Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("object_bind_group"),
        });

        Self {
            model,
            position,
            rotation,
            uniform_buffer,
            bind_group,
        }
    }
}

pub trait DrawObject<'a> {
    fn draw_mesh(
        &mut self,
        mesh: &'a Mesh,
        camera_bind_group: &'a wgpu::BindGroup,
        diffuse_bind_group: &'a wgpu::BindGroup,
        translation_bind_group: &'a wgpu::BindGroup,
    );
    fn draw_object(
        &mut self,
        object: &'a Object,
        camera_bind_group: &'a wgpu::BindGroup,
        material_store: &'a MaterialStore,
        texture_store: &'a TextureStore,
    );
}

impl<'a, 'b> DrawObject<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh(
        &mut self,
        mesh: &'b Mesh,
        camera_bind_group: &'b wgpu::BindGroup,
        diffuse_bind_group: &'b wgpu::BindGroup,
        translation_bind_group: &'b wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, &camera_bind_group, &[]);
        self.set_bind_group(1, &diffuse_bind_group, &[]);
        self.set_bind_group(2, &translation_bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, 0..1);
    }

    fn draw_object(
        &mut self,
        object: &'a Object,
        camera_bind_group: &'a wgpu::BindGroup,
        material_store: &'a MaterialStore,
        texture_store: &'a TextureStore,
    ) {
        for data in &object.model.data {
            let mesh = &data.mesh;
            let material = material_store.get_material(data.material_id);
            let diffuse = texture_store.get_texture(material.diffuse_texture_id);
            self.draw_mesh(
                mesh,
                camera_bind_group,
                diffuse.bind_group.as_ref().unwrap(),
                &object.bind_group,
            );
        }
    }
}

pub struct ObjectManager {
    pub bind_group_layout: wgpu::BindGroupLayout,
    actors: Vec<Object>,            // User-defined objects
    immutable_objects: [Object; 1], // Static objects
}

#[allow(dead_code)]
impl<'a> ObjectManager {
    pub async fn new(context: &'a Context<'a>, material_store: &'a mut MaterialStore) -> Self {
        let obj_path = Path::new("models/cube.obj").to_path_buf();
        let obj_model = load_model(&obj_path, &context.device, material_store)
            .await
            .unwrap();

        let bind_group_layout =
            context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("object_bind_group_layout"),
                });

        let immutable_objects = [Object::new(
            obj_model,
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0)),
            &context.device,
            &bind_group_layout,
        )];

        Self {
            bind_group_layout: bind_group_layout,
            actors: Vec::new(),
            immutable_objects: immutable_objects,
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
