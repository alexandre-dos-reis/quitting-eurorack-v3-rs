use crate::config::env_vars::load_env_vars;
use api::types::ApiResponse;
use axum::{extract::State, routing::get, Router};
use config::env_vars::EnvVars;
use log::info;
use maud::Markup;
use templates::pages::home::home_page;

use std::sync::Arc;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
// use tower_http::services::ServeDir;
mod api;
mod config;
mod templates;

async fn home_handler(State(state): State<Arc<EnvVars>>) -> Markup {
    let res = reqwest::Client::new()
        .get(state.api_endpoint.clone() + "/items/module?fields=*,pictures.directus_files_id")
        .header("Authorization", state.api_key.clone())
        .send()
        .await
        .unwrap()
        .json::<ApiResponse>()
        .await
        .unwrap();

    home_page(res.data)
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("starting up");

    let env_var = load_env_vars();
    let port = env_var.port;
    let shared_state = Arc::new(env_var);

    let app = Router::new()
        .route("/", get(home_handler))
        // .nest_service("/assets", get_service(ServeDir::new("./src/assets/dist")))
        .with_state(shared_state);

    axum::Server::bind(&SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port,
    ))
    .serve(app.into_make_service())
    .await
    .unwrap();
    info!("Server launch on http://localhost:{:?}", port);
}
