use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Modifier {
  Alt,
  Command,
  Control,
  Shift,
}

impl fmt::Display for Modifier {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Modifier::Alt => write!(f, "Modifier::Alt"),
      Modifier::Command => write!(f, "Modifier::Command"),
      Modifier::Control => write!(f, "Modifier::Control"),
      Modifier::Shift => write!(f, "Modifier::Shift"),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Key {
  ArrowDown,
  ArrowLeft,
  ArrowRight,
  ArrowUp,
  Backspace,
  Character(&'static str),
  Delete,
  End,
  Enter,
  Escape,
  Home,
  PageDown,
  PageUp,
  Space,
  Tab,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
}

impl fmt::Display for Key {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Key::ArrowDown => write!(f, "Key::ArrowDown"),
      Key::ArrowLeft => write!(f, "Key::ArrowLeft"),
      Key::ArrowRight => write!(f, "Key::ArrowRight"),
      Key::ArrowUp => write!(f, "Key::ArrowUp"),
      Key::Backspace => write!(f, "Key::Backspace"),
      Key::Character(char) => write!(f, "Key::Character::{}", char),
      Key::Delete => write!(f, "Key::Delete"),
      Key::End => write!(f, "Key::End"),
      Key::Enter => write!(f, "Key::Enter"),
      Key::Escape => write!(f, "Key::Escape"),
      Key::Home => write!(f, "Key::Home"),
      Key::PageDown => write!(f, "Key::PageDown"),
      Key::PageUp => write!(f, "Key::PageUp"),
      Key::Space => write!(f, "Key::Space"),
      Key::Tab => write!(f, "Key::Tab"),
      Key::F1 => write!(f, "Key::F1"),
      Key::F2 => write!(f, "Key::F2"),
      Key::F3 => write!(f, "Key::F3"),
      Key::F4 => write!(f, "Key::F4"),
      Key::F5 => write!(f, "Key::F5"),
      Key::F6 => write!(f, "Key::F6"),
      Key::F7 => write!(f, "Key::F7"),
      Key::F8 => write!(f, "Key::F8"),
      Key::F9 => write!(f, "Key::F9"),
      Key::F10 => write!(f, "Key::F10"),
      Key::F11 => write!(f, "Key::F11"),
      Key::F12 => write!(f, "Key::F12"),
    }
  }
}
