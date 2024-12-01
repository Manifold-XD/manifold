use crate::renderer::Renderer;

use std::sync::Arc;

use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct ManifoldApp {
    pub window: Option<Arc<Window>>,
    pub size: PhysicalSize<u32>,
    pub renderer: Option<Renderer>,
    pub cursor_position: Option<(f64, f64)>,
}

impl ManifoldApp {
    pub fn new() -> ManifoldApp {
        Self {
            window: None,
            size: PhysicalSize::new(0, 0),
            renderer: None,
            cursor_position: None,
        }
    }
}
