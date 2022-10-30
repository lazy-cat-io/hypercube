fn main() {
  println!("Hello, hypercube!");
}

#[allow(dead_code)]
fn square(x: i32) -> i32 {
  x.pow(2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn square_test() {
    assert_eq!(square(2), 4);
  }
}
