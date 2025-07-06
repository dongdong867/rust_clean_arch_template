use auth::application::error::AuthUseCaseError;

pub enum VerifyError {
    InavalidToken,
}

impl From<AuthUseCaseError> for VerifyError {
    fn from(error: AuthUseCaseError) -> Self {
        match error {
            AuthUseCaseError::InvalidIdToken => VerifyError::InavalidToken,
        }
    }
}
