use async_trait::async_trait;

use crate::adapter::{
    dto::authenticated_user_dto::AuthenticatedUserDto,
    port::service::{auth_service::AuthService, auth_service_error::AuthServiceError},
};

#[derive(Clone)]
pub struct AuthFirebaseServiceImpl {}

impl AuthFirebaseServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl AuthService for AuthFirebaseServiceImpl {
    async fn verify_id_token(
        &self,
        id_token: &str,
    ) -> Result<AuthenticatedUserDto, AuthServiceError> {
        println!("{}", id_token);
        todo!()
    }
}
