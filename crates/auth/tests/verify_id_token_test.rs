use std::sync::Arc;

use auth::{
    adapter::{
        dto::AuthenticatedUserDto, error::AuthProviderError, repository::AuthRepositoryImpl,
    },
    application::{
        error::AuthUseCaseError,
        port::r#in::{VerifyIdTokenCommand, VerifyIdTokenUseCase},
        service::VerifyIdTokenUseCaseImpl,
    },
    domain::AuthenticatedUser,
};
use mock::mock_auth_port::MockAuthProvider;
use mockall::predicate::*;

mod mock;

const VALID_ID_TOKEN: &str = "valid_id_token";
const INVALID_ID_TOKEN: &str = "invalid_id_token";

fn create_mock_service() -> MockAuthProvider {
    let authenticated_user = AuthenticatedUserDto {
        id: "user_id".to_string(),
        email: "user@example.com".to_string(),
        name: "User Name".to_string(),
    };

    let mut mock = MockAuthProvider::new();
    mock.expect_verify_id_token()
        .with(eq(VALID_ID_TOKEN))
        .return_const(Ok(authenticated_user));
    mock.expect_verify_id_token()
        .with(eq(INVALID_ID_TOKEN))
        .return_const(Err(AuthProviderError::InvalidCredentials));

    mock
}

fn setup() -> Arc<dyn VerifyIdTokenUseCase> {
    let auth_service = Arc::new(create_mock_service());
    let auth_repository = Arc::new(AuthRepositoryImpl::new(auth_service));
    Arc::new(VerifyIdTokenUseCaseImpl::new(auth_repository))
}

#[actix_web::test]
async fn user_can_get_authenticated_user_with_valid_id_token() {
    // Given user passing a id token to the use case
    // When the id token is valid
    // Then should return an authenticated user object

    let expected_user = AuthenticatedUser {
        id: "user_id".to_string(),
        email: "user@example.com".to_string(),
        name: "User Name".to_string(),
    };

    let verify_id_token_use_case = setup();
    let result = verify_id_token_use_case
        .execute(VerifyIdTokenCommand::new(VALID_ID_TOKEN.to_string()))
        .await
        .unwrap();

    assert_eq!(result, expected_user);
}

#[actix_web::test]
async fn user_will_receive_invalid_id_token_error_when_id_token_is_invalid() {
    // Given user passing a id token to the use case
    // When the id token is invalid
    // Then should return an error

    let verify_id_token_use_case = setup();
    let result = verify_id_token_use_case
        .execute(VerifyIdTokenCommand::new(INVALID_ID_TOKEN.to_string()))
        .await
        .unwrap_err();

    assert_eq!(result, AuthUseCaseError::InvalidIdToken);
}

#[actix_web::test]
async fn user_will_receive_invalid_input_error_when_id_token_is_empty() {
    // Given user passing an empty id token to the use case
    // When the id token is empty
    // Then should return an error

    let verify_id_token_use_case = setup();
    let result = verify_id_token_use_case
        .execute(VerifyIdTokenCommand::new("".to_string()))
        .await
        .unwrap_err();

    assert_eq!(
        result,
        AuthUseCaseError::InvalidInput("ID token cannot be empty".to_string())
    );
}
