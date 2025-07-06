use crate::application::error::auth_use_case_error::AuthUseCaseError;
use crate::domain::AuthenticatedUser;

use async_trait::async_trait;

#[async_trait]
pub trait VerifyIdTokenUseCase: Send + Sync {
    async fn execute(&self, id_token: &str) -> Result<AuthenticatedUser, AuthUseCaseError>;
}
