use async_trait::async_trait;
use auth::AuthService;
use auth::adapter::dto::authenticated_user_dto::AuthenticatedUserDto;
use auth::adapter::port::service::auth_service_error::AuthServiceError;
use mockall::mock;

mock! {
    pub AuthService {}

    #[async_trait]
    impl AuthService for AuthService {
            async fn verify_id_token(&self, id_token: &str) -> Result<AuthenticatedUserDto, AuthServiceError>;
        }
}
