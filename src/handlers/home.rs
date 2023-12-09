use std::sync::Arc;

use crate::{
    api::types::{ApiResponse, Module},
    config::env_vars::EnvVars,
    templates::pages::{error_page::error_page, home_page::home_page},
};
use axum::extract::State;
use maud::Markup;

pub async fn home_handler(State(state): State<Arc<EnvVars>>) -> Markup {
    let api_endpoint = &state.api_endpoint;

    let res = reqwest::Client::new()
        .get(api_endpoint.to_owned() + "/items/module?fields=*,pictures.directus_files_id")
        .header("Authorization", state.api_key.clone())
        .send()
        .await;

    match res {
        Err(_) => error_page(),
        Ok(r) => match r.json::<ApiResponse<Module>>().await {
            Err(_) => error_page(),
            Ok(json) => home_page(json.data, &(api_endpoint.to_owned() + "/assets")),
        },
    }
}
