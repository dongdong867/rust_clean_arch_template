use crate::application::error::AuthUseCaseError;

#[derive(Debug, PartialEq)]
pub enum AuthRepositoryError {
    InvalidIdToken,
}

impl AuthRepositoryError {
    pub fn to_use_case_error(&self) -> AuthUseCaseError {
        match self {
            AuthRepositoryError::InvalidIdToken => AuthUseCaseError::InvalidIdToken,
        }
    }
}
