mod rect;

pub use rect::Rect;

pub trait Geometry {
  fn render(&self);
}
