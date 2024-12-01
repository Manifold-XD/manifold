use std::sync::Arc;
use winit::window::Window;

pub struct Context<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

pub async fn init_wgpu(window: Arc<Window>) -> Context<'static> {
    let size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let surface = instance.create_surface(window).unwrap();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
                label: None,
            },
            None,
        )
        .await
        .unwrap();

    let surface_capabilities = surface.get_capabilities(&adapter);
    let surface_format = surface_capabilities
        .formats
        .iter()
        .copied()
        .filter(|f| f.is_srgb())
        .next()
        .unwrap_or(surface_capabilities.formats[0]);

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_capabilities.present_modes[0],
        alpha_mode: surface_capabilities.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &config);

    Context {
        surface: surface,
        device: device,
        queue: queue,
        config: config,
    }
}
