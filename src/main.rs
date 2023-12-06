use crate::config::env_vars::load_env_vars;
use api::types::{Module, Response};
use askama::Template;
use axum::{extract::State, routing::get, Router};
use config::env_vars::EnvVars;
use std::sync::Arc;

mod api;
mod config;

#[tokio::main]
async fn main() {
    let env_var = load_env_vars();
    let shared_state = Arc::new(env_var.clone());
    let app = Router::new()
        .route("/", get(hello))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{:?}", env_var.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "home.html")]
struct HelloTemplate<'a> {
    modules: Vec<Module>,
    title: &'a str,
}

async fn hello(State(state): State<Arc<EnvVars>>) -> HelloTemplate<'static> {
    let res = reqwest::Client::new()
        .get(state.api_endpoint.clone() + "/items/module?fields=*,pictures.directus_files_id")
        .header("Authorization", state.api_key.clone())
        .send()
        .await
        // TODO: handle error
        .unwrap()
        .json::<Response>()
        .await
        // TODO: handle error
        .unwrap();

    HelloTemplate {
        modules: res.data,
        title: "👋 Hello, I'm quitting Eurorack. 😭",
    }
}
