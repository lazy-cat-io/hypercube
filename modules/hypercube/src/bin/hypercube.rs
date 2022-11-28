use clap::Parser;
use tracing::info;

use hypercube::{cli, config, editor};

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  let args = cli::Args::parse();

  info!("config path: {:?}", config::config_dir());
  info!("cli args: {:?}", args);

  editor::launch(args).await;
}
