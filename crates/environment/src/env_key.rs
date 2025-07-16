#[derive(Debug, PartialEq)]
pub enum EnvKey {
    Mode,
    Test,
}

impl EnvKey {
    pub fn as_str(&self) -> &str {
        match self {
            EnvKey::Mode => "MODE",
            EnvKey::Test => "TEST",
        }
    }
}
