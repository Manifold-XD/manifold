use super::ManifoldApp;

use super::rendering::Renderer;
use super::window_management::WindowManager;

use winit::event::*;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, NamedKey};

use log::{debug, info, warn};

pub trait EventHandler {
    fn handle_window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent);
    fn handle_keyboard_input(&mut self, event_loop: &ActiveEventLoop, key: &Key);
}

impl EventHandler for ManifoldApp {
    fn handle_window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        debug!("{event:?}");
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                self.resize(physical_size.width, physical_size.height);
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                let size = self.size.as_ref().unwrap();
                let scaled_width = (size.width as f64 * scale_factor).floor() as u32;
                let scaled_height = (size.height as f64 * scale_factor).floor() as u32;
                self.resize(scaled_width, scaled_height);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: key,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => self.handle_keyboard_input(event_loop, &key),
            WindowEvent::RedrawRequested => {
                self.render();
            }
            _ => (),
        }
    }

    fn handle_keyboard_input(&mut self, event_loop: &ActiveEventLoop, key: &Key) {
        match key.as_ref() {
            Key::Named(NamedKey::Space) => {
                info!("Space!");
            }
            Key::Named(NamedKey::Escape) => {
                warn!("Escape pressed, exiting the application.");
                event_loop.exit();
            }
            _ => (),
        }
    }
}
