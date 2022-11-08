use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Kind {
  Clojure,
  PlainText,
  Rust,
}

impl fmt::Display for Kind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Kind::Clojure => write!(f, "language::Kind::Clojure"),
      Kind::PlainText => write!(f, "language::Kind::PlainText"),
      Kind::Rust => write!(f, "language::Kind::Rust"),
    }
  }
}
