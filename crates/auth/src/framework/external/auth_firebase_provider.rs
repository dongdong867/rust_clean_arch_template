use async_trait::async_trait;

use crate::adapter::{
    dto::authenticated_user_dto::AuthenticatedUserDto, error::AuthProviderError, port::out::AuthProvider,
};

#[derive(Clone)]
pub struct AuthFirebaseServiceImpl {}

impl Default for AuthFirebaseServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthFirebaseServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl AuthProvider for AuthFirebaseServiceImpl {
    async fn verify_id_token(
        &self,
        id_token: &str,
    ) -> Result<AuthenticatedUserDto, AuthProviderError> {
        let authenticated_user_dto = AuthenticatedUserDto {
            id: "user_id".to_string(),
            email: "email".to_string(),
            name: "name".to_string(),
        };
        match id_token {
            "valid id token" => Ok(authenticated_user_dto),
            _ => Err(AuthProviderError::InvalidCredentials),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const VALID_ID_TOKEN: &str = "valid id token";
    const INVALID_ID_TOKEN: &str = "invalid id token";

    #[actix_web::test]
    async fn test_verify_success() {
        let expected_user = AuthenticatedUserDto {
            id: "user_id".to_string(),
            email: "email".to_string(),
            name: "name".to_string(),
        };

        let auth_firebase_service = AuthFirebaseServiceImpl::new();
        let result = auth_firebase_service
            .verify_id_token(VALID_ID_TOKEN)
            .await
            .unwrap();
        assert_eq!(result, expected_user)
    }

    #[actix_web::test]
    async fn test_verify_failure() {
        let expected_error = AuthProviderError::InvalidCredentials;

        let auth_firebase_service = AuthFirebaseServiceImpl::new();
        let result = auth_firebase_service
            .verify_id_token(INVALID_ID_TOKEN)
            .await
            .unwrap_err();
        assert_eq!(result, expected_error)
    }
}
