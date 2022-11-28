mod button;

pub use button::Button;

pub trait Component {
  fn render(&self);
}

pub enum LifeCycle {
  DidMount,
  DidUpdate,
  Render,
  WillUnmount,
}

pub struct Props<T: Component> {
  pub id: u64,
  pub name: String,
  pub children: Vec<T>,
}
