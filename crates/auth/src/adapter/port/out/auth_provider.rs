use async_trait::async_trait;

use crate::adapter::dto::authenticated_user_dto::AuthenticatedUserDto;
use crate::adapter::error::AuthProviderError;

#[async_trait]
pub trait AuthProvider: Send + Sync {
    async fn verify_id_token(
        &self,
        id_token: &str,
    ) -> Result<AuthenticatedUserDto, AuthProviderError>;
}
