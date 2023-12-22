use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::{env, str::FromStr};
use strum_macros::EnumString;

const FALLBACK_PORT: u16 = 3000;

#[derive(Debug, Clone, Copy, EnumString, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum AppEnv {
    Dev,
    Prod,
    Ci,
    Test,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnvVars {
    pub app_env: AppEnv,
    pub app_port: u16,
    pub api_endpoint: String,
    pub api_key: String,
    pub contact_email: String,
}

lazy_static! {
    pub static ref ENV_VARS: EnvVars = {
        let app_env = AppEnv::from_str(&env::var("APP_ENV").unwrap_or(String::from("Dev")))
            .unwrap_or(AppEnv::Dev);

        let port: u16 = match env::var("PORT") {
            Ok(val) => val.parse::<u16>().unwrap_or(FALLBACK_PORT),
            Err(_) => FALLBACK_PORT,
        };

        if app_env == AppEnv::Dev {
            if env::var("API_ENDPOINT").is_ok()
                || env::var("API_KEY").is_ok()
                || env::var("CONTACT_EMAIL").is_ok()
            {
                panic!("Env vars must be provided via the .env file !")
            }
            dotenv().expect(".env file not found !");
        }

        return EnvVars {
            app_env,
            app_port: port,
            api_endpoint: env::var("API_ENDPOINT")
                .expect("API_ENDPOINT need to be set in a production environment !"),
            api_key: env::var("API_KEY").expect("API KEY is mandatory !"),
            contact_email: env::var("CONTACT_EMAIL").expect("CONTACT_EMAIL is mandatory !"),
        };
    };
}
