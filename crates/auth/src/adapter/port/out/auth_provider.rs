use crate::adapter::{dto::authenticated_user_dto::AuthenticatedUserDto, error::AuthProviderError};
use async_trait::async_trait;

#[async_trait]
pub trait AuthProvider: Send + Sync {
    async fn verify_id_token(
        &self,
        id_token: &str,
    ) -> Result<AuthenticatedUserDto, AuthProviderError>;
}
