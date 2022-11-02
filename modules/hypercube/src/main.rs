use tracing_subscriber;

use hypercube::editor::engine;

fn main() {
  tracing_subscriber::fmt::init();
  println!("Hello, {}!", engine::Engine::Hypercube);
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
