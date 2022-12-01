use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddr};
use tokio::signal;
use tracing::info;
use uuid::Uuid;

use crate::cli;

#[derive(Debug, Deserialize)]
struct Command {
  name: String,
}

#[derive(Debug, Serialize)]
struct Event {
  id: String,
  command: String,
}

async fn invoke(Json(payload): Json<Command>) -> impl IntoResponse {
  let event = Event { id: Uuid::new_v4().to_string(), command: payload.name };
  info!("response: {:?}", event);
  (StatusCode::ACCEPTED, Json(event))
}

async fn shutdown_signal() {
  let ctrl_c = async {
    signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
  };

  #[cfg(unix)]
  let terminate = async {
    signal::unix::signal(signal::unix::SignalKind::terminate()).expect("failed to install signal handler").recv().await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  tokio::select! {
      _ = ctrl_c => {},
      _ = terminate => {},
  }

  info!("signal received, starting graceful shutdown");
}

pub struct Server {}

impl Server {
  pub fn new() -> Self {
    Server {}
  }

  pub async fn start(&self) {
    let app = Router::new().route("/invoke", post(invoke));

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 2022);
    info!("server listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).with_graceful_shutdown(shutdown_signal()).await.unwrap();
  }
}

impl Default for Server {
  fn default() -> Self {
    Self::new()
  }
}

pub async fn start(args: cli::Args) {
  info!("server launch!");
  info!("received args: {:?}", args);
  Server::new().start().await;
}
