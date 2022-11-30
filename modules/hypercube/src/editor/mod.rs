use std::fmt;
use tracing::info;

use super::{cli, ui};

#[derive(Debug, Clone, Copy)]
pub enum Engine {
  Default,
  Hypercube,
}

impl fmt::Display for Engine {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Engine::Default => write!(f, "Engine::Default"),
      Engine::Hypercube => write!(f, "Engine::Hypercube"),
    }
  }
}

pub fn start(args: cli::Args) {
  info!("hypercube launch!");
  info!("received args: {:?}", args);
  let button = ui::component::Button::new();
  let rect = ui::geometry::Rect::new();
  ui::component::Component::render(&button);
  ui::geometry::Geometry::render(&rect);

  ui::window::Renderer::new().run();
}
