use crate::config::env_vars::load_env_vars;
use api::types::ApiResponse;
use axum::{
    extract::State,
    response::Html,
    routing::{get, get_service},
    Router,
};
use config::env_vars::EnvVars;
use dioxus::prelude::*;
use log::info;
use std::sync::Arc;
use tower_http::services::ServeDir;

mod api;
mod config;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("starting up");

    let env_var = load_env_vars();
    let port = env_var.app_port;
    let shared_state = Arc::new(env_var);

    let app = Router::new()
        .route("/", get(home))
        .nest_service("/assets", get_service(ServeDir::new("./src/assets/dist")))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(format!("localhost:{:?}", port))
        .await
        .unwrap();

    info!("Server launch on {:?}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

pub fn render(f: LazyNodes<'_, '_>) -> Html<String> {
    Html(dioxus_ssr::render_lazy(f))
}

async fn home(State(state): State<Arc<EnvVars>>) -> Html<String> {
    let res = reqwest::Client::new()
        .get(state.api_endpoint.clone() + "/items/module?fields=*,pictures.directus_files_id")
        .header("Authorization", state.api_key.clone())
        .send()
        .await;

    match res {
        Err(_) => render(rsx!( div { "not ok" })),
        Ok(r) => match r.json::<ApiResponse>().await {
            Err(_) => render(rsx!( div { "not ok" })),
            Ok(json) => render(rsx!( div {
                for m in json.data {
                    div {
                        class: "bg-red-500",
                        {m.manufacturer}
                    }
                }
            })),
        },
    }
}
