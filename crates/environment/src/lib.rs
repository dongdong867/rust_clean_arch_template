use std::collections::HashMap;
use std::env;

pub(crate) mod env_error;
pub(crate) mod env_key;

pub use env_error::EnvError;
pub use env_key::EnvKey;

pub struct Environment {
    variables: HashMap<String, String>,
}

impl Environment {
    pub fn new() -> Result<Self, EnvError> {
        let mode = env::var(EnvKey::Mode.as_str())
            .ok()
            .unwrap_or("DEV".to_string());
        let env_path = match mode.as_str() {
            "DEV" => ".dev.env",
            "PROD" => ".prod.env",
            _ => return Err(EnvError::FileNotExist),
        };

        let _ = dotenvy::from_filename_override(env_path);
        let variables = env::vars().collect();

        Ok(Environment { variables })
    }

    pub fn get(&self, key: EnvKey) -> Result<String, EnvError> {
        let key_str = key.as_str();
        self.variables
            .get(key_str)
            .cloned()
            .ok_or_else(|| EnvError::KeyNotFound(key_str.to_string()))
    }
}
