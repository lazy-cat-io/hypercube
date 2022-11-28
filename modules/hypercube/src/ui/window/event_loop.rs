use pollster;
use tracing::*;

use tao::{
  dpi::PhysicalSize,
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::{Window, WindowBuilder},
};

struct Setup {
  size: PhysicalSize<u32>,
  surface: wgpu::Surface,
  adapter: wgpu::Adapter,
  device: wgpu::Device,
  queue: wgpu::Queue,
}

async fn setup(window: &Window) -> Setup {
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

  {
    let adapter_info = adapter.get_info();
    info!("Using {} ({:?})", adapter_info.name, adapter_info.backend);
  }

  let trace_dir = std::env::var("HYPERCUBE_WGPU_TRACE");
  let (device, queue) = adapter
    .request_device(
      &wgpu::DeviceDescriptor { label: None, features: wgpu::Features::default(), limits: wgpu::Limits::default() },
      trace_dir.ok().as_ref().map(std::path::Path::new),
    )
    .await
    .expect("Unable to find a suitable GPU adapter!");

  Setup { size, surface, adapter, device, queue }
}

pub fn run() {
  let event_loop = EventLoop::new();

  let window_title = String::from("::hypercube");
  let builder = WindowBuilder::new().with_title(&window_title);
  let window = builder.build(&event_loop).unwrap();

  let setup = pollster::block_on(setup(&window));
  let surface = setup.surface;
  let device = setup.device;
  let size = setup.size;
  let adapter = setup.adapter;
  let _queue = setup.queue;

  let mut config = wgpu::SurfaceConfiguration {
    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    format: surface.get_supported_formats(&adapter)[0],
    width: size.width,
    height: size.height,
    present_mode: wgpu::PresentMode::Fifo,
    alpha_mode: wgpu::CompositeAlphaMode::Auto,
  };
  surface.configure(&device, &config);

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
        info!("The close button was pressed. Hypercube is stopping...");
        *control_flow = ControlFlow::Exit
      }

      Event::WindowEvent {
        event: WindowEvent::Resized(size) | WindowEvent::ScaleFactorChanged { new_inner_size: &mut size, .. },
        ..
      } => {
        info!("Resizing to {:?}", size);
        config.width = size.width.max(1);
        config.height = size.height.max(1);
        surface.configure(&device, &config);
        window.request_redraw();
      }

      Event::MainEventsCleared => {
        window.request_redraw();
      }

      _ => (),
    }
  });
}
