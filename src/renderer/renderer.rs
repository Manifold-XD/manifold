use super::camera::Camera;
use super::context;
use super::material::MaterialStore;
use super::object::{DrawObject, ObjectManager};
use super::pipeline::PipelineStore;
use super::shader::ShaderStore;
use super::texture::TextureStore;

use std::path::PathBuf;
use std::sync::Arc;
use winit::window::Window;

pub struct Renderer {
    context: context::Context<'static>,
    camera: Camera,
    #[allow(unused)]
    shader_store: ShaderStore,
    texture_store: TextureStore,
    material_store: MaterialStore,
    object_manager: ObjectManager,
    pipeline_store: PipelineStore,
}

impl Renderer {
    pub async fn setup(window: Arc<Window>) -> Self {
        let context = context::init_wgpu(window).await;

        let camera = Camera::new(&context.device, &context.config);
        let shader_store = ShaderStore::new(&context);
        let mut texture_store = TextureStore::new(&context);
        texture_store
            .load_texture_from_file(PathBuf::from("textures/rickroll.jpg"), &context)
            .await;
        let mut material_store = MaterialStore::new();
        let mut object_manager = ObjectManager::new(&context, &mut material_store).await;

        let pipeline_store = PipelineStore::new(
            &context,
            &shader_store,
            &[
                &camera.bind_group_layout,
                &texture_store.bind_group_layout,
                &object_manager.bind_group_layout,
            ],
        );

        object_manager
            .create_actor(
                &PathBuf::from("models/cube.obj"),
                &context,
                &mut material_store,
            )
            .await;

        Self {
            context,
            camera,
            shader_store,
            texture_store,
            material_store,
            object_manager,
            pipeline_store,
        }
    }

    fn update(&mut self) {
        self.camera.update(&self.context.queue);
        self.object_manager.update(&self.context.queue);
    }

    pub fn render(&mut self) {
        self.update();

        let frame = match self.context.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Lost) => {
                self.context
                    .surface
                    .configure(&self.context.device, &self.context.config);
                self.context
                    .surface
                    .get_current_texture()
                    .expect("Failed to acquire next surface texture after reconfigure")
            }
            Err(wgpu::SurfaceError::Outdated) => {
                self.context
                    .surface
                    .configure(&self.context.device, &self.context.config);
                return;
            }
            Err(e) => {
                eprintln!("Failed to acquire next surface texture: {:?}", e);
                return;
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.01,
                            g: 0.01,
                            b: 0.01,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                timestamp_writes: None,
                occlusion_query_set: None,
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.texture_store.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
            });

            for object in self.object_manager.iter() {
                render_pass.set_pipeline(&self.pipeline_store.basic);
                render_pass.draw_object(
                    object,
                    &self.camera.bind_group,
                    &self.material_store,
                    &self.texture_store,
                    &self.pipeline_store,
                );
            }
        }

        self.context.queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.context.config.width = width;
        self.context.config.height = height;
        self.context
            .surface
            .configure(&self.context.device, &self.context.config);
        self.camera.eye.aspect = width as f32 / height as f32;
        self.texture_store.depth_texture = TextureStore::create_depth_texture(&self.context);
    }

    pub fn handle_camera_movement(&mut self, key_event: winit::event::KeyEvent) {
        self.camera.controller.process_input_events(&key_event);
    }

    pub fn handle_mouse_delta(&mut self, delta: (f64, f64)) {
        self.camera.controller.process_mouse_delta(delta);
    }
}
