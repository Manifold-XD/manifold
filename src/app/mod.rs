use std::sync::Arc;
use winit::window::Window;

pub struct ManifoldApp {
    window: Option<Arc<Window>>,
    size: Option<winit::dpi::PhysicalSize<u32>>,
    instance: Option<wgpu::Instance>,
    surface: Option<wgpu::Surface<'static>>,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    config: Option<wgpu::SurfaceConfiguration>,
}

impl ManifoldApp {
    pub fn new() -> ManifoldApp {
        Self {
            window: None,
            size: None,
            instance: None,
            surface: None,
            device: None,
            queue: None,
            config: None,
        }
    }
}

pub mod event_handler;
pub mod rendering;
pub mod window_management;
pub mod winit_handler;