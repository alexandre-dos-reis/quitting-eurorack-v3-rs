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
use maud::{html, Markup};
use std::sync::Arc;
use tower_http::services::ServeDir;
mod api;
mod config;

async fn hello_world() -> Markup {
    html! {
        h1 { "Hello, World!" }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("starting up");

    let env_var = load_env_vars();
    let port = env_var.port;

    let shared_state = Arc::new(env_var);
    let app = Router::new()
        .route("/", get(hello_world))
        .nest_service("/assets", get_service(ServeDir::new("./src/assets/dist")))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(format!("localhost:{:?}", port))
        .await
        .unwrap();

    info!("Server launch on {:?}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// #[derive(Template)]
// #[template(path = "pages/home.html")]
// struct HomeTemplate {
//     modules: Vec<Module>,
//     is_ok: bool,
// }

// async fn home(State(state): State<Arc<EnvVars>>) -> HomeTemplate {
//     let res = reqwest::Client::new()
//         .get(state.api_endpoint.clone() + "/items/module?fields=*,pictures.directus_files_id")
//         .header("Authorization", state.api_key.clone())
//         .send()
//         .await;
//
//     match res {
//         Err(_) => HomeTemplate {
//             modules: vec![],
//             is_ok: false,
//         },
//         Ok(r) => match r.json::<ApiResponse>().await {
//             Err(_) => HomeTemplate {
//                 modules: vec![],
//                 is_ok: false,
//             },
//             Ok(json) => HomeTemplate {
//                 modules: json.data,
//                 is_ok: true,
//             },
//         },
//     }
// }
