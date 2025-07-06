use crate::application::error::AuthUseCaseError;

#[derive(Clone, Debug, PartialEq)]
pub enum AuthControllerError {
    InvalidToken,
}

impl From<AuthUseCaseError> for AuthControllerError {
    fn from(value: AuthUseCaseError) -> Self {
        match value {
            AuthUseCaseError::InvalidIdToken => AuthControllerError::InvalidToken,
        }
    }
}
