use clap::Parser;
use tracing::info;

use hypercube::{
  cli::{self, Mode},
  config,
  system::{client, server},
};

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let args = cli::Args::parse();

  info!("config path: {:?}", config::config_dir());

  match args.mode {
    Mode::Client => client::start(args).await,
    Mode::Server => server::start(args).await,
  }
}
