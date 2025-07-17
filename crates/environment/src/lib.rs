use std::env;
use std::sync::OnceLock;

use crate::env_key::EnvKey;

pub(crate) mod env_error;
pub(crate) mod env_key;

pub use env_error::EnvError;

pub struct Environment {
    pub test: String,
}

impl Environment {
    fn load() -> Result<Self, EnvError> {
        let mode = std::env::var(EnvKey::Mode.as_str()).unwrap_or_else(|_| "DEV".to_string());
        let env_path = match mode.as_str() {
            "DEV" => ".dev.env",
            "PROD" => ".prod.env",
            _ => return Err(EnvError::UnexpectedMode),
        };

        let _ = dotenvy::from_filename_override(env_path).map_err(|_| EnvError::FileNotExist);
        Ok(Self {
            test: Self::get(EnvKey::Test.as_str())?,
        })
    }

    fn get(key: &str) -> Result<String, EnvError> {
        env::var(key).map_err(|_| EnvError::KeyNotFound(key.to_string()))
    }
}

static ENVIRONMENT: OnceLock<Environment> = OnceLock::new();

pub fn env() -> &'static Environment {
    ENVIRONMENT.get_or_init(|| Environment::load().expect("Failed to load environment."))
}
