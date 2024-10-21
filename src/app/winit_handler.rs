use super::ManifoldApp;
use crate::renderer::Renderer;

use super::event_handler::EventHandler;
use super::window_management::WindowManager;

pub use winit::application::ApplicationHandler;
use winit::event::{StartCause, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

use log::debug;

impl ApplicationHandler for ManifoldApp {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        debug!("new_events: {cause:?}");
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.setup_window(event_loop);

        let future_renderer = Renderer::setup(self.window.clone().unwrap());
        self.renderer = Some(pollster::block_on(future_renderer))
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        self.handle_window_event(event_loop, event);
    }
}
