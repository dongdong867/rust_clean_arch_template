use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::{
        error::auth_use_case_error::AuthUseCaseError,
        port::{
            r#in::use_case::verify_id_token_use_case::VerifyIdTokenUseCase,
            out::repository::auth_repository::AuthRepository,
        },
    },
    domain::AuthenticatedUser,
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
            .map_err(|repository_error| repository_error.to_use_case_error())
    }
}
