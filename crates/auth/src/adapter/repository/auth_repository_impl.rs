use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    adapter::port::service::auth_service::AuthService,
    application::port::repository::{
        auth_repository::AuthRepository, auth_repository_error::AuthRepositoryError,
    },
    AuthenticatedUser,
};

#[derive(Clone)]
pub struct AuthRepositoryImpl {
    auth_service: Arc<dyn AuthService>,
}

impl AuthRepositoryImpl {
    pub fn new(auth_service: Arc<dyn AuthService>) -> Self {
        Self { auth_service }
    }
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn verify_id_token(
        &self,
        id_token: &str,
    ) -> Result<AuthenticatedUser, AuthRepositoryError> {
        self.auth_service
            .verify_id_token(id_token)
            .await
            .map_err(|service_error| service_error.to_repository_error())
            .map(|dto| dto.into())
    }
}
