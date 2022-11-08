use std::fmt;

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
