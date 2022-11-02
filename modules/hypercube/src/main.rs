use tracing_subscriber;

use hypercube::editor::engine::Engine;
use hypercube::ui::keyboard::Key;

fn main() {
  tracing_subscriber::fmt::init();
  println!("{}", Key::Character("X"));
  println!("{}", Engine::Hypercube);
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
