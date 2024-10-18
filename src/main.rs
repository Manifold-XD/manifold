mod app;
use app::ManifoldApp;

use winit::event_loop::EventLoop;

fn main() -> Result<(), impl std::error::Error> {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let mut app = ManifoldApp::new();
    event_loop.run_app(&mut app)
}
