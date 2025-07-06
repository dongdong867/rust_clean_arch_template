use crate::application::port::out::repository::auth_repository_error::AuthRepositoryError;

#[derive(Clone, Debug, PartialEq)]
pub enum AuthServiceError {
    InvalidCredentials,
}

impl AuthServiceError {
    pub fn to_repository_error(&self) -> AuthRepositoryError {
        match self {
            AuthServiceError::InvalidCredentials => AuthRepositoryError::InvalidIdToken,
        }
    }
}
