use crate::auth::application::port::repository::auth_repository_error::AuthRepositoryError;

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
