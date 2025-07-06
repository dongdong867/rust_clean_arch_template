use crate::application::error::auth_use_case_error::AuthUseCaseError;
use crate::application::port::r#in::VerifyIdTokenCommand;
use crate::domain::AuthenticatedUser;

use async_trait::async_trait;

#[async_trait]
pub trait VerifyIdTokenUseCase: Send + Sync {
    async fn execute(
        &self,
        command: VerifyIdTokenCommand,
    ) -> Result<AuthenticatedUser, AuthUseCaseError>;
}
