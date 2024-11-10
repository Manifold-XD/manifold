use crate::renderer::Renderer;

use std::sync::Arc;

use winit::dpi::PhysicalSize;
use winit::window::Window;

pub struct ManifoldApp {
    window: Option<Arc<Window>>,
    size: PhysicalSize<u32>,
    renderer: Option<Renderer<'static>>,
    cursor_position: Option<(f64, f64)>,
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

pub mod event_handler;
pub mod window_management;
pub mod winit_handler;
