use crate::{config::env_vars::load_env_vars, handlers::home::home_handler};
use axum::routing::get_service;
use axum::{routing::get, Router};
use log::info;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tower_http::services::ServeDir;

mod api;
mod config;
mod handlers;
mod templates;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("starting up");

    let env_var = load_env_vars();
    let port = env_var.app_port;

    let router = Router::new()
        .route("/", get(home_handler))
        .nest_service("/assets", get_service(ServeDir::new("./src/assets/dist")))
        .with_state(Arc::new(env_var));

    let server = axum::Server::bind(&SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port,
    ))
    .serve(router.into_make_service());

    info!("Server launch on http://localhost:{:?}", port);

    server.await.unwrap();
}
