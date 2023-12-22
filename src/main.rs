use crate::{
    config::env_vars::{AppEnv, ENV_VARS},
    handlers::home::home_handler,
};
use axum::routing::get_service;
use axum::{routing::get, Router};
use log::info;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tower_http::services::ServeDir;

mod api;
mod config;
mod handlers;
mod templates;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("starting up");

    let router = Router::new()
        .route("/", get(home_handler))
        .nest_service("/assets", get_service(ServeDir::new("./public")));

    let ip = if ENV_VARS.app_env == AppEnv::Prod {
        Ipv4Addr::new(0, 0, 0, 0)
    } else {
        Ipv4Addr::new(127, 0, 0, 1)
    };

    let server = axum::Server::bind(&SocketAddr::new(IpAddr::V4(ip), ENV_VARS.app_port))
        .serve(router.into_make_service());

    info!("Server launch on http://localhost:{:?}", ENV_VARS.app_port);
    server.await.unwrap();
}
