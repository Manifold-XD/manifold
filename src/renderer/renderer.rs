use super::util::resources;

use super::camera::Camera;
use super::context;
use super::model::{DrawModel, Model};
use super::pipeline;
use super::texture::TexturePool;

use std::{path::Path, sync::Arc};
use winit::window::Window;

pub struct Renderer {
    context: context::Context<'static>,
    pipeline: wgpu::RenderPipeline,
    camera: Camera,
    texture_pool: TexturePool,
    obj_model: Model,
}

impl Renderer {
    pub async fn setup(window: Arc<Window>) -> Self {
        let context = context::init_wgpu(window).await;

        let camera: Camera = Camera::new(&context.device, &context.config);
        let texture_pool = TexturePool::new(&context);

        let pipeline = pipeline::init_pipeline(
            &context,
            &[&camera.bind_group_layout, &texture_pool.bind_group_layout],
        );

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
            context: context,
            pipeline: pipeline,
            camera: camera,
            texture_pool: texture_pool,
            obj_model: obj_model,
        }
    }

    fn update(&mut self) {
        self.camera.update();
        self.context.queue.write_buffer(
            &self.camera.buffer,
            0,
            bytemuck::cast_slice(&[self.camera.uniform]),
        );
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
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                timestamp_writes: None,
                occlusion_query_set: None,
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.texture_pool.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_vertex_buffer(0, self.obj_model.meshes[0].vertex_buffer.slice(..));
            render_pass.set_pipeline(&self.pipeline);
            render_pass.draw_model(&self.obj_model, &self.camera.bind_group);
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
        self.texture_pool.depth_texture = TexturePool::create_depth_texture(&self.context);
    }

    pub fn handle_camera_movement(&mut self, key_event: winit::event::KeyEvent) {
        self.camera.controller.process_input_events(&key_event);
    }

    pub fn handle_mouse_delta(&mut self, delta: (f64, f64)) {
        self.camera.controller.process_mouse_delta(delta);
    }
}
