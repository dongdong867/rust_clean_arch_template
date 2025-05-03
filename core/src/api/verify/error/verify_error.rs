use auth::application::error::auth_use_case_error::AuthUseCaseError;


pub enum VerifyError {
    InavalidToken,
}

impl From<AuthUseCaseError> for VerifyError {
    fn from(error: AuthUseCaseError) -> Self {
        match error {
            AuthUseCaseError::RepositoryError(_) => VerifyError::InavalidToken,
        }
    }
}
