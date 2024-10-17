use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId};

use log::{debug, info, warn};

fn main() -> Result<(), impl std::error::Error> {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let mut app = ManifoldApp::default();
    event_loop.run_app(&mut app)
}

#[derive(Default)]
struct ManifoldApp {
    window: Option<Window>,
}

impl ApplicationHandler for ManifoldApp {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        debug!("new_events: {cause:?}");
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("Manifold");
        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        debug!("{event:?}");

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: key,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match key.as_ref() {
                Key::Named(NamedKey::Space) => {
                    info!("Space!");
                }
                Key::Named(NamedKey::Escape) => {
                    warn!("Escape pressed, exiting the application.");
                    event_loop.exit();
                }
                _ => (),
            },
            WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();
                window.pre_present_notify();
            }
            _ => (),
        }
    }
}
