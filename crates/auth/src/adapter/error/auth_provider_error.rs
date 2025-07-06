use crate::application::port::out::repository::auth_repository_error::AuthRepositoryError;

#[derive(Clone, Debug, PartialEq)]
pub enum AuthProviderError {
    InvalidCredentials,
}

impl AuthProviderError {
    pub fn to_repository_error(&self) -> AuthRepositoryError {
        match self {
            AuthProviderError::InvalidCredentials => AuthRepositoryError::InvalidIdToken,
        }
    }
}
