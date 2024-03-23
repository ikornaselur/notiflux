use anyhow::{Context, Result};
use base64::prelude::*;
use std::env;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_public_key: Vec<u8>,
    pub host: String,
    pub port: u16,
    pub worker_count: usize,
}

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_WORKER_COUNT: usize = 4;

static CONFIG: OnceLock<Config> = OnceLock::new();

impl Config {
    pub fn init_from_env() -> Result<Self> {
        let jwt_public_key_b64 =
            env::var("JWT_PUBLIC_KEY_B64").context("Unable to read JWT_PUBLIC_KEY_B64 from env")?;
        let jwt_public_key = BASE64_STANDARD
            .decode(jwt_public_key_b64.as_bytes())
            .context("Unable to Base64 decode JWT_PUBLIC_KEY_B64")?;

        let port = env::var("PORT")
            .unwrap_or_else(|_| DEFAULT_PORT.to_string())
            .parse::<u16>()?;
        let host = env::var("HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string());
        let worker_count = env::var("WORKER_COUNT")
            .unwrap_or_else(|_| DEFAULT_WORKER_COUNT.to_string())
            .parse::<usize>()?;

        Ok(Config {
            jwt_public_key,
            host,
            port,
            worker_count,
        })
    }
}

pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| Config::init_from_env().expect("Failed to initialize config"))
}
