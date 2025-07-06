use async_trait::async_trait;

use crate::domain::AuthenticatedUser;

use super::auth_repository_error::AuthRepositoryError;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn verify_id_token(
        &self,
        id_token: &str,
    ) -> Result<AuthenticatedUser, AuthRepositoryError>;
}
