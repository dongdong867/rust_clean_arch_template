use async_trait::async_trait;
use auth::AuthPort;
use auth::adapter::dto::AuthenticatedUserDto;
use auth::adapter::error::AuthServiceError;
use mockall::mock;

mock! {
    pub AuthProvider {}

    #[async_trait]
    impl AuthPort for AuthProvider {
            async fn verify_id_token(
                &self,
                id_token: &str
            ) -> Result<AuthenticatedUserDto, AuthServiceError>;
        }
}
