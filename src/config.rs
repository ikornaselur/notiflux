use base64::prelude::*;
use std::env;
use std::sync::OnceLock;

use crate::{NotifluxError, NotifluxErrorType};

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
    pub fn init_from_env() -> Result<Self, NotifluxError> {
        let jwt_public_key_b64 = env::var("JWT_PUBLIC_KEY_B64").map_err(|_| NotifluxError {
            message: Some("JWT_PUBLIC_KEY_B64 is not set in env".to_string()),
            error_type: NotifluxErrorType::EnvError,
        })?;

        let jwt_public_key = BASE64_STANDARD
            .decode(jwt_public_key_b64.as_bytes())
            .map_err(|_| NotifluxError {
                message: Some("Unable to Base64 decode JWT_PUBLIC_KEY_B64".to_string()),
                error_type: NotifluxErrorType::Base64DecodeError,
            })?;
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
    CONFIG.get_or_init(|| match Config::init_from_env() {
        Ok(config) => config,
        Err(e) => {
            log::error!("Error initializing config: {}", e);
            std::process::exit(1);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_from_env() {
        env::set_var("JWT_PUBLIC_KEY_B64", "SGVsbG8hCg==");
        env::set_var("PORT", "1234");
        env::set_var("HOST", "10.11.12.13");
        env::set_var("WORKER_COUNT", "4");

        let config = Config::init_from_env().unwrap();

        assert_eq!(config.port, 1234);
        assert_eq!(config.host, "10.11.12.13");
        assert_eq!(config.worker_count, 4);
        assert_eq!(config.jwt_public_key, b"Hello!\n".to_vec());
    }

    #[test]
    fn test_init_defaults() {
        env::set_var("JWT_PUBLIC_KEY_B64", "SGVsbG8hCg==");

        let config = Config::init_from_env().unwrap();

        assert_eq!(config.port, DEFAULT_PORT);
        assert_eq!(config.host, DEFAULT_HOST);
        assert_eq!(config.worker_count, DEFAULT_WORKER_COUNT);
        assert_eq!(config.jwt_public_key, b"Hello!\n".to_vec());
    }

    #[test]
    fn test_init_requires_jwt_public_key() {
        env::remove_var("JWT_PUBLIC_KEY_B64");

        let config = Config::init_from_env();

        assert!(config.is_err());
        let err = config.unwrap_err();
        assert_eq!(err.error_type, NotifluxErrorType::EnvError);
        assert_eq!(
            err.message,
            Some("JWT_PUBLIC_KEY_B64 is not set in env".to_string())
        );
    }
}
