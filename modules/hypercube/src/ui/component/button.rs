use super::Component;

#[derive(Debug, Clone, Copy)]
pub struct Button;

impl Button {
  pub fn new() -> Self {
    Button {}
  }
}

impl Default for Button {
  fn default() -> Self {
    Self::new()
  }
}

impl Component for Button {
  fn render(&self) {
    println!("render Button")
  }
}
