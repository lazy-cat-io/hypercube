use tao::{
  event::{ElementState, Event, WindowEvent},
  event_loop,
  keyboard::Key as KeyPress,
  window::{Theme, Window, WindowBuilder},
};

use pollster;
use tracing::info;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use crate::ui::{
  keyboard::{Key, Modifier},
  wgpu::Wgpu,
};

#[derive(Debug)]
pub enum HypercubeEvent {}

pub struct Renderer {
  pub event_loop: event_loop::EventLoop<HypercubeEvent>,
  pub event_loop_proxy: event_loop::EventLoopProxy<HypercubeEvent>,
  pub wgpu: Wgpu,
  pub window: Window,
}

impl Default for Renderer {
  fn default() -> Self {
    Self::new()
  }
}

impl Renderer {
  pub fn new() -> Self {
    let event_loop = event_loop::EventLoop::<HypercubeEvent>::with_user_event();
    let event_loop_proxy = event_loop.create_proxy();
    let window_title = String::from("::hypercube");
    let builder = WindowBuilder::new().with_title(&window_title).with_decorations(false).with_theme(Some(Theme::Light));
    let window = builder.build(&event_loop).unwrap();

    apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
      .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    let wgpu = pollster::block_on(Wgpu::init(&window));
    Self { event_loop, event_loop_proxy, wgpu, window }
  }

  pub fn run(self) {
    let event_loop = self.event_loop;
    let window = self.window;
    let wgpu = self.wgpu;

    let surface = wgpu.surface;
    let device = wgpu.device;
    let _size = wgpu.size;
    let _adapter = wgpu.adapter;
    let _queue = wgpu.queue;
    let mut config = wgpu.surface_configuration;

    event_loop.run(move |event, _, control_flow| {
      *control_flow = event_loop::ControlFlow::Wait;

      match event {
        // Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => {
        //   let scale = window.scale_factor() as f32;
        //   let x = position.x as f32 / scale;
        //   let y = (config.height as f32 - position.y as f32) / scale;
        //   info!("mouse position: {:?}", [x, y])
        // }

        // Event::WindowEvent { event: WindowEvent::MouseInput { state, button, .. }, .. } => {
        //   info!("mouse input: state: {:?}, button: {:?}", state, button);
        // }
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

        // Event::MenuEvent { menu_id, .. } => {
        //   info!("menu event: {:?}", menu_id)
        // }
        Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *control_flow = event_loop::ControlFlow::Exit,

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
}
