use crate::{api::types::Response, config::env_vars::load_env_vars};
mod api;
mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_var = load_env_vars();

    let res = reqwest::blocking::Client::new()
        .get(env_var.api_endpoint + "/items/module?fields=*,pictures.directus_files_id")
        .header("Authorization", env_var.api_key)
        .send()?
        .json::<Response>();

    if let Err(err) = res {
        panic!("Bad Request with {}", err);
    };

    res.unwrap()
        .data
        .iter()
        .for_each(|m| println!("{}: {}", m.manufacturer, m.name));
    Ok(())
}
