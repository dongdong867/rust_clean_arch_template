use crate::application::error::AuthUseCaseError;

#[derive(Debug, PartialEq)]
pub enum VerifyIdTokenCommandError {
    EmptyToken,
}

impl VerifyIdTokenCommandError {
    pub fn to_use_case_error(&self) -> AuthUseCaseError {
        match self {
            VerifyIdTokenCommandError::EmptyToken => {
                AuthUseCaseError::InvalidInput("ID token cannot be empty".to_string())
            },
        }
    }
}
