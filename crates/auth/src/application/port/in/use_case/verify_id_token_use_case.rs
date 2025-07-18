use async_trait::async_trait;

use crate::application::error::auth_use_case_error::AuthUseCaseError;
use crate::application::port::r#in::VerifyIdTokenCommand;
use crate::domain::AuthenticatedUser;

#[async_trait]
pub trait VerifyIdTokenUseCase: Send + Sync {
    async fn execute(
        &self,
        command: VerifyIdTokenCommand,
    ) -> Result<AuthenticatedUser, AuthUseCaseError>;
}
