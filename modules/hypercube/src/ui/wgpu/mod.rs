use std::{env, path::Path};
use tao::{dpi::PhysicalSize, window::Window};
use tracing::info;
use wgpu::{Adapter, Device, Queue, Surface, SurfaceConfiguration};

pub struct Wgpu {
  pub adapter: Adapter,
  pub device: Device,
  pub queue: Queue,
  pub size: PhysicalSize<u32>,
  pub surface: Surface,
  pub surface_configuration: SurfaceConfiguration,
}

impl Wgpu {
  pub async fn init(window: &Window) -> Self {
    let backend = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
    let instance = wgpu::Instance::new(backend);

    let (size, surface) = unsafe {
      let size = window.inner_size();
      let surface = instance.create_surface(&window);
      (size, surface)
    };

    let adapter = wgpu::util::initialize_adapter_from_env_or_default(&instance, backend, Some(&surface))
      .await
      .expect("No suitable GPU adapters found on the system!");

    let info = adapter.get_info();
    info!("Using {} ({:?})", info.name, info.backend);

    let trace_dir = env::var("HYPERCUBE_WGPU_TRACE");
    let wgpu_descriptor =
      wgpu::DeviceDescriptor { label: None, features: wgpu::Features::default(), limits: wgpu::Limits::default() };

    let (device, queue) = adapter
      .request_device(&wgpu_descriptor, trace_dir.ok().as_ref().map(Path::new))
      .await
      .expect("Unable to find a suitable GPU adapter!");

    let surface_configuration = SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface.get_supported_formats(&adapter)[0],
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode: wgpu::CompositeAlphaMode::Auto,
    };
    surface.configure(&device, &surface_configuration);

    Self { adapter, device, queue, size, surface, surface_configuration }
  }
}
