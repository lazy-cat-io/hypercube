use tracing::info;

use crate::{cli, editor};

// TODO: [2022-11-30, Ilshat Sultanov] We need to do some research. Can't we run the server and client together?
// thread 'tokio-runtime-worker' panicked at 'On macOS, `EventLoop` must be created on the main thread!'

pub async fn start(args: cli::Args) {
  info!("client launch!");
  info!("received args: {:?}", args);
  editor::start(args);
}
