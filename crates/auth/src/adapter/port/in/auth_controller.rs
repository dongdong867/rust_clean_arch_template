use async_trait::async_trait;

use crate::adapter::dto::VerifyIdTokenRequest;
use crate::adapter::dto::verify_id_token_response::VerifyIdTokenResponse;
use crate::adapter::error::auth_controller_error::AuthControllerError;

#[async_trait]
pub trait AuthController: Send + Sync {
    async fn verify_id_token(
        &self,
        request: VerifyIdTokenRequest,
    ) -> Result<VerifyIdTokenResponse, AuthControllerError>;
}
