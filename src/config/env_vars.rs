use dotenvy::dotenv;
use std::{env, str::FromStr};
use strum_macros::EnumString;

#[derive(EnumString)]
pub enum AppEnv {
    #[strum(serialize = "Dev", ascii_case_insensitive)]
    Dev,

    #[strum(serialize = "Prod", ascii_case_insensitive)]
    Prod,
}

#[derive(Debug)]
pub struct EnvVars {
    pub api_endpoint: String,
    pub port: usize,
}

const DEFAULT_PORT: usize = 3000;

pub fn load_env_vars() -> EnvVars {
    let env =
        AppEnv::from_str(&env::var("ENV").unwrap_or(String::from("Dev"))).unwrap_or(AppEnv::Dev);

    let port: usize = match env::var("PORT") {
        Ok(val) => val.parse::<usize>().unwrap_or(DEFAULT_PORT),
        Err(_) => DEFAULT_PORT,
    };

    match env {
        AppEnv::Prod => EnvVars {
            api_endpoint: env::var("API_ENDPOINT")
                .expect("API_ENDPOINT need to be set in a proudction environment !"),
            port,
        },
        AppEnv::Dev => EnvVars {
            api_endpoint: {
                if env::var("API_ENDPOINT").is_ok() {
                    panic!("API_ENDPOINT env variable must be provided via the .env file !")
                }
                dotenv().expect(".env file not found !");
                env::var("API_ENDPOINT").expect("dotenv didn't work !")
            },
            port,
        },
    }
}
