use tracing::info;

use super::Geometry;

#[derive(Debug, Clone, Copy)]
pub struct Rect;

impl Rect {
  pub fn new() -> Self {
    Rect {}
  }
}

impl Default for Rect {
  fn default() -> Self {
    Self::new()
  }
}

impl Geometry for Rect {
  fn render(&self) {
    info!("render Rect")
  }
}
