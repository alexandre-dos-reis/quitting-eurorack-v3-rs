use crate::config::env_vars::load_env_vars;

mod config;

fn main() {
    let env_var = load_env_vars();

    println!("{}", env_var.api_endpoint)
}
