#[derive(Debug, PartialEq)]
pub enum AuthUseCaseError {
    InvalidIdToken,
    InvalidInput(String),
}
