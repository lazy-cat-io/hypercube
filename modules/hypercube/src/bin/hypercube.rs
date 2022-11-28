use clap::Parser;

use hypercube::{cli, config, editor};

fn main() {
  let args = cli::Args::parse();

  println!("config path: {:?}", config::config_dir());
  println!("cli args: {:?}", args);

  editor::launch(args);
}
