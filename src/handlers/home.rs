use crate::{
    api::types::{ApiResponse, Module},
    config::env_vars::ENV_VARS,
    templates::pages::{error_page::error_page, home_page::home_page},
};
use maud::Markup;

pub async fn home_handler() -> Markup {
    let res = reqwest::Client::new()
        .get(ENV_VARS.api_endpoint.to_owned() + "/items/module?fields=*,pictures.directus_files_id")
        .header("Authorization", &ENV_VARS.api_key)
        .send()
        .await;

    match res {
        Err(_) => error_page(),
        Ok(r) => match r.json::<ApiResponse<Module>>().await {
            Err(_) => error_page(),
            Ok(json) => home_page(json.data),
        },
    }
}
