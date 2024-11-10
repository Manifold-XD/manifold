use super::ManifoldApp;

use super::window_management::WindowManager;

use winit::event::*;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, NamedKey};

use log::{debug, info, warn};

pub trait EventHandler {
    fn handle_window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent);
    fn handle_keyboard_input(&mut self, event_loop: &ActiveEventLoop, key_event: KeyEvent);
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
                let scaled_width = (self.size.width as f64 * scale_factor).floor() as u32;
                let scaled_height = (self.size.height as f64 * scale_factor).floor() as u32;
                self.resize(scaled_width, scaled_height);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.handle_keyboard_input(event_loop, event);
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
                self.renderer.as_mut().unwrap().render();
            }
            _ => (),
        }
    }

    fn handle_keyboard_input(&mut self, event_loop: &ActiveEventLoop, key_event: KeyEvent) {
        match key_event.logical_key.as_ref() {
            Key::Named(NamedKey::Space) => {
                if key_event.state == ElementState::Pressed {
                    info!("Space!");
                }
            }
            Key::Named(NamedKey::Escape) => {
                if key_event.state == ElementState::Pressed {
                    warn!("Escape pressed, exiting the application.");
                    event_loop.exit();
                }
            }
            Key::Character("w")
            | Key::Character("s")
            | Key::Character("a")
            | Key::Character("d") => {
                self.renderer
                    .as_mut()
                    .unwrap()
                    .handle_camera_input(key_event);
            }
            _ => (),
        }
    }
}
