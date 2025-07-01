use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    AuthenticatedUser, application::error::auth_use_case_error::AuthUseCaseError,
    application::port::out::repository::auth_repository::AuthRepository,
};



pub struct VerifyIdTokenUseCaseImpl {
    auth_repository: Arc<dyn AuthRepository>,
}

impl VerifyIdTokenUseCaseImpl {
    pub fn new(auth_repository: Arc<dyn AuthRepository>) -> Self {
        Self { auth_repository }
    }
}

#[async_trait]
impl VerifyIdTokenUseCase for VerifyIdTokenUseCaseImpl {
    async fn execute(&self, id_token: &str) -> Result<AuthenticatedUser, AuthUseCaseError> {
        self.auth_repository
            .verify_id_token(id_token)
            .await
            .map_err(AuthUseCaseError::RepositoryError)
    }
}
