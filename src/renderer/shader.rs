use super::context;
use crate::res_path;

#[allow(unused)]
pub enum ShaderType {
    Basic,
    Hyper,
}

pub struct ShaderStore {
    pub basic: wgpu::ShaderModule, // 3D
    pub hyper: wgpu::ShaderModule, // 4D
}

impl ShaderStore {
    pub fn new(context: &context::Context) -> Self {
        let basic = context
            .device
            .create_shader_module(wgpu::include_wgsl!(res_path!("shaders/basic.wgsl")));
        let hyper = context
            .device
            .create_shader_module(wgpu::include_wgsl!(res_path!("shaders/hyper.wgsl")));

        Self { basic, hyper }
    }
}
