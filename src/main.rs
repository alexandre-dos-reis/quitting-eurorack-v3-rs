use crate::config::env_vars::load_env_vars;
use api::types::{ApiResponse, Module};
use askama::Template;
use axum::{
    extract::State,
    routing::{get, get_service},
    Router,
};
use config::env_vars::EnvVars;
use log::info;
use std::sync::Arc;
use tower_http::services::ServeDir;

mod api;
mod config;

#[tokio::main]
async fn main() {
    env_logger::init();

    let env_var = load_env_vars();
    info!("starting up");

    let shared_state = Arc::new(env_var.clone());
    let app = Router::new()
        .route("/", get(home))
        .nest_service("/assets", get_service(ServeDir::new("./src/assets/dist")))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(format!("localhost:{:?}", env_var.port))
        .await
        .unwrap();

    info!("Server launch on {:?}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HelloTemplate<'a> {
    modules: Vec<Module>,
    title: &'a str,
}

async fn home(State(state): State<Arc<EnvVars>>) -> HelloTemplate<'static> {
    let res = reqwest::Client::new()
        .get(state.api_endpoint.clone() + "/items/module?fields=*,pictures.directus_files_id")
        .header("Authorization", state.api_key.clone())
        .send()
        .await;

    match res {
        Err(_) => HelloTemplate {
            modules: vec![],
            title: "An error occured, please come back later...",
        },
        Ok(r) => match r.json::<ApiResponse>().await {
            Err(_) => HelloTemplate {
                modules: vec![],
                title: "An error occured, please come back later...",
            },
            Ok(json) => HelloTemplate {
                modules: json.data,
                title: "👋 Hello, I'm quitting Eurorack. 😭",
            },
        },
    }
}
