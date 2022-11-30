use clap::{Parser, ValueEnum};

#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
  #[clap(long, value_enum, default_value_t = Mode::Client)]
  pub mode: Mode,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Mode {
  Client,
  Server,
}
