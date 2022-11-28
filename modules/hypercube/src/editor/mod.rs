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

pub async fn launch(args: cli::Args) {
  info!("hypercube launch!");
  info!("received args: {:?}", args);
  let button = ui::component::Button::new();
  let rect = ui::geometry::Rect::new();
  ui::Component::render(&button);
  ui::Geometry::render(&rect);
  ui::window::run();
}
