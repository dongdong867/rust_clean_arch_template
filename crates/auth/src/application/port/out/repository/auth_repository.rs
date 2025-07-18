use async_trait::async_trait;

use crate::application::port::out::AuthRepositoryError;
use crate::domain::AuthenticatedUser;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn verify_id_token(
        &self,
        id_token: &str,
    ) -> Result<AuthenticatedUser, AuthRepositoryError>;
}
