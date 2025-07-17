use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    adapter::{
        dto::{verify_id_token_response::VerifyIdTokenResponse, VerifyIdTokenRequest},
        error::auth_controller_error::AuthControllerError, port::r#in::AuthController,
    },
    application::port::r#in::{VerifyIdTokenCommand, VerifyIdTokenUseCase},
};

pub struct AuthControllerImpl {
    verify_id_token_use_case: Arc<dyn VerifyIdTokenUseCase>,
}

impl AuthControllerImpl {
    pub fn new(verify_id_token_use_case: Arc<dyn VerifyIdTokenUseCase>) -> Self {
        Self {
            verify_id_token_use_case,
        }
    }
}

#[async_trait]
impl AuthController for AuthControllerImpl {
    async fn verify_id_token(
        &self,
        _request: VerifyIdTokenRequest,
    ) -> Result<VerifyIdTokenResponse, AuthControllerError> {
        let result = self
            .verify_id_token_use_case
            .execute(VerifyIdTokenCommand::new(_request.id_token))
            .await
            .map_err(AuthControllerError::from);

        match result {
            Ok(user) => Ok(VerifyIdTokenResponse::from(user)),
            Err(error) => Err(error),
        }
    }
}
