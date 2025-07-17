#[derive(Debug)]
pub enum EnvError {
    FileNotExist,
    KeyNotFound(String),
    UnexpectedMode
}
