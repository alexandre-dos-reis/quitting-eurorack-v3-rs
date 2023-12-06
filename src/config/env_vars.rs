use dotenvy::dotenv;
use std::{env, str::FromStr};
use strum_macros::EnumString;

#[derive(Debug, Clone, Copy, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum AppEnv {
    Dev,
    Prod,
    Ci,
}

#[derive(Debug, Clone)]
pub struct EnvVars {
    pub app_env: AppEnv,
    pub app_port: usize,
    pub api_endpoint: String,
    pub api_key: String,
}

const DEFAULT_PORT: usize = 3000;
const API_ENDPOINT_VAR: &str = "API_ENDPOINT";

// TODO: make en vars a global object: see lazy static
pub fn load_env_vars() -> EnvVars {
    let app_env =
        AppEnv::from_str(&env::var("ENV").unwrap_or(String::from("Dev"))).unwrap_or(AppEnv::Dev);

    let port: usize = match env::var("PORT") {
        Ok(val) => val.parse::<usize>().unwrap_or(DEFAULT_PORT),
        Err(_) => DEFAULT_PORT,
    };

    match app_env {
        AppEnv::Prod => EnvVars {
            app_port: port,
            app_env,
            api_key: env::var("API_KEY").expect("API KEY is mandatory !"),
            api_endpoint: env::var(API_ENDPOINT_VAR)
                .expect("API_ENDPOINT need to be set in a production environment !"),
        },
        AppEnv::Dev => {
            if env::var(API_ENDPOINT_VAR).is_ok() || env::var("api_key").is_ok() {
                panic!("Env variables must be provided via the .env file !")
            }
            dotenv().expect(".env file not found !");
            EnvVars {
                app_port: port,
                app_env,
                api_key: env::var("API_KEY").expect("dotenv didn't work !"),
                api_endpoint: env::var(API_ENDPOINT_VAR).expect("dotenv didn't work !"),
            }
        }
        AppEnv::Ci => EnvVars {
            app_port: port,
            app_env,
            api_key: env::var("API_KEY").expect("API KEY is mandatory !"),
            api_endpoint: env::var(API_ENDPOINT_VAR)
                .expect("API_ENDPOINT need to be set in a CI environment !"),
        },
    }
}
