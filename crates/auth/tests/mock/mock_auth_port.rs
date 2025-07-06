use async_trait::async_trait;
use auth::AuthProvider;
use auth::adapter::dto::AuthenticatedUserDto;
use auth::adapter::error::AuthProviderError;
use mockall::mock;

mock! {
    pub AuthProvider {}

    #[async_trait]
    impl AuthProvider for AuthProvider {
            async fn verify_id_token(
                &self,
                id_token: &str
            ) -> Result<AuthenticatedUserDto, AuthProviderError>;
        }
}
