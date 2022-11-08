use hypercube::{
  editor, language,
  ui::{self, component, geometry, keyboard},
};
use tracing_subscriber as ts;

fn main() {
  ts::fmt::init();
  println!("{:?}", keyboard::Key::Character("X"));
  println!("{:?}", editor::Engine::Hypercube);
  println!("{:?}", language::Kind::Clojure);
  println!("{:?}", component::Button::new());
  println!("{:?}", geometry::Rect::new());
  let button = component::Button::new();
  let rect = geometry::Rect::new();
  ui::Component::render(&button);
  ui::Geometry::render(&rect);
}

#[cfg(test)]
mod tests {
  fn square(x: i32) -> i32 {
    x.pow(2)
  }

  #[test]
  fn square_test() {
    assert_eq!(square(2), 4);
  }
}
