use async_trait::async_trait;
use auth::AuthService;
use auth::adapter::dto::AuthenticatedUserDto;
use auth::adapter::error::AuthServiceError;
use mockall::mock;

mock! {
    pub AuthService {}

    #[async_trait]
    impl AuthService for AuthService {
            async fn verify_id_token(&self, id_token: &str) -> Result<AuthenticatedUserDto, AuthServiceError>;
        }
}
