use std::fmt;

use super::context;
use crate::res_path;

#[allow(unused)]
pub enum ShaderType {
    Grid,
    Basic,
    Hyper,
}

impl fmt::Display for ShaderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShaderType::Grid => write!(f, "grid"),
            ShaderType::Basic => write!(f, "basic"),
            ShaderType::Hyper => write!(f, "hyper"),
        }
    }
}

pub struct ShaderStore {
    pub grid: wgpu::ShaderModule,  // editor grid
    pub basic: wgpu::ShaderModule, // 3D
    pub hyper: wgpu::ShaderModule, // 4D
}

impl ShaderStore {
    pub fn new(context: &context::Context) -> Self {
        let grid = context
            .device
            .create_shader_module(wgpu::include_wgsl!(res_path!("shaders/editor_grid.wgsl")));
        let basic = context
            .device
            .create_shader_module(wgpu::include_wgsl!(res_path!("shaders/basic.wgsl")));
        let hyper = context
            .device
            .create_shader_module(wgpu::include_wgsl!(res_path!("shaders/hyper.wgsl")));

        Self { grid, basic, hyper }
    }
}
