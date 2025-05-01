use async_trait::async_trait;

use crate::auth::adapter::dto::authenticated_user_dto::AuthenticatedUserDto;

use super::auth_service_error::AuthServiceError;

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn verify_id_token(
        &self,
        id_token: &str,
    ) -> Result<AuthenticatedUserDto, AuthServiceError>;
}
