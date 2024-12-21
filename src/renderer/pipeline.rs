use super::{
    context::Context,
    model::{ModelVertex, Vertex},
    shader::ShaderStore,
    texture::Texture,
};

pub fn init_pipeline(
    context: &Context,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
    shader: &wgpu::ShaderModule,
) -> wgpu::RenderPipeline {
    let render_pipeline_layout =
        context
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: bind_group_layouts,
                push_constant_ranges: &[],
            });

    let render_pipeline = context
        .device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[ModelVertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: context.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

    render_pipeline
}

#[allow(unused)]
pub struct PipelineStore {
    pub basic: wgpu::RenderPipeline,
    pub hyper: wgpu::RenderPipeline,
}

impl PipelineStore {
    pub fn new(
        context: &Context,
        shader_store: &ShaderStore,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> Self {
        let basic = init_pipeline(context, bind_group_layouts, &shader_store.basic);
        let hyper = init_pipeline(context, bind_group_layouts, &shader_store.hyper);

        Self { basic, hyper }
    }
}
