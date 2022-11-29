use pollster;
use tracing::*;

use tao::{
  dpi::PhysicalSize,
  event::{ElementState, Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  keyboard::Key as KeyPress,
  window::{Window, WindowBuilder},
};

use crate::ui::keyboard::{Key, Modifier};

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

#[derive(Debug)]
pub enum HypercubeEvent {}

pub fn run() {
  let event_loop = EventLoop::<HypercubeEvent>::with_user_event();

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
      Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => {
        let scale = window.scale_factor() as f32;
        let x = position.x as f32 / scale;
        let y = (config.height as f32 - position.y as f32) / scale;
        info!("mouse position: {:?}", [x, y])
      }

      Event::WindowEvent { event: WindowEvent::MouseInput { state, button, .. }, .. } => {
        info!("mouse input: state: {:?}, button: {:?}", state, button);
      }

      Event::WindowEvent { event: WindowEvent::KeyboardInput { event, .. }, .. } => {
        if event.state == ElementState::Pressed {
          let key = match event.logical_key {
            KeyPress::Character(c) => Some(Key::Character(c)),
            KeyPress::Enter => Some(Key::Enter),
            KeyPress::Tab => Some(Key::Tab),
            KeyPress::Space => Some(Key::Space),
            KeyPress::ArrowDown => Some(Key::ArrowDown),
            KeyPress::ArrowLeft => Some(Key::ArrowLeft),
            KeyPress::ArrowRight => Some(Key::ArrowRight),
            KeyPress::ArrowUp => Some(Key::ArrowUp),
            KeyPress::End => Some(Key::End),
            KeyPress::Home => Some(Key::Home),
            KeyPress::PageDown => Some(Key::PageDown),
            KeyPress::PageUp => Some(Key::PageUp),
            KeyPress::Backspace => Some(Key::Backspace),
            KeyPress::Delete => Some(Key::Delete),
            KeyPress::Escape => Some(Key::Escape),
            KeyPress::F1 => Some(Key::F1),
            KeyPress::F2 => Some(Key::F2),
            KeyPress::F3 => Some(Key::F3),
            KeyPress::F4 => Some(Key::F4),
            KeyPress::F5 => Some(Key::F5),
            KeyPress::F6 => Some(Key::F6),
            KeyPress::F7 => Some(Key::F7),
            KeyPress::F8 => Some(Key::F8),
            KeyPress::F9 => Some(Key::F9),
            KeyPress::F10 => Some(Key::F10),
            KeyPress::F11 => Some(Key::F11),
            KeyPress::F12 => Some(Key::F12),
            _ => None,
          };

          info!("key: {:?}", key)
        }
      }

      Event::WindowEvent { event: WindowEvent::ModifiersChanged(mods), .. } => {
        if mods.shift_key() {
          info!("shift: {:?}", Modifier::Shift);
        }
        if mods.control_key() {
          info!("control: {:?}", Modifier::Control)
        }
        if mods.shift_key() {
          info!("shift: {:?}", Modifier::Shift)
        }
        if mods.alt_key() {
          info!("alt: {:?}", Modifier::Alt)
        }
        if mods.super_key() {
          info!("command: {:?}", Modifier::Command)
        }
      }

      Event::UserEvent(event) => {
        info!("user event: {:?}", event);
      }

      Event::WindowEvent {
        event: WindowEvent::Resized(size) | WindowEvent::ScaleFactorChanged { new_inner_size: &mut size, .. },
        ..
      } => {
        info!("resize {:?}", size);
        config.width = size.width.max(1);
        config.height = size.height.max(1);
        surface.configure(&device, &config);
        window.request_redraw();
      }

      Event::MenuEvent { menu_id, .. } => {
        info!("menu event: {:?}", menu_id)
      }

      Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *control_flow = ControlFlow::Exit,

      Event::MainEventsCleared => {
        window.request_redraw();
      }

      // Event::RedrawRequested(_) => {
      //   let window_size = window.inner_size();
      //   let scale = window.scale_factor() as f32;
      //   let width = window_size.width as f32 / scale;
      //   let height = window_size.height as f32 / scale;
      //   info!("redraw: width: {:?}, height: {:?}", width, height)
      // }
      _ => (),
    }
  });
}
