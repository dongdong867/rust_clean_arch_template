use auth::{
    adapter::{
        controller::AuthControllerImpl,
        dto::{AuthenticatedUserDto, VerifyIdTokenRequest, VerifyIdTokenResponse},
        error::{AuthControllerError, AuthProviderError},
        port::r#in::AuthController,
        repository::AuthRepositoryImpl,
    },
    application::service::VerifyIdTokenUseCaseImpl,
};
use chrono::Utc;
use mock::mock_auth_port::MockAuthProvider;
use mockall::predicate::*;
use std::sync::Arc;

mod mock;

const VALID_ID_TOKEN: &str = "valid_id_token";
const INVALID_ID_TOKEN: &str = "invalid_id_token";

fn create_mock_auth_provider() -> MockAuthProvider {
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

fn setup() -> Arc<dyn AuthController> {
    let auth_service = Arc::new(create_mock_auth_provider());
    let auth_repository = Arc::new(AuthRepositoryImpl::new(auth_service));
    let verify_id_token_use_case = Arc::new(VerifyIdTokenUseCaseImpl::new(auth_repository.clone()));
    Arc::new(AuthControllerImpl::new(verify_id_token_use_case))
}

#[actix_web::test]
async fn user_can_get_authenticated_user_with_valid_id_token() {
    // Given user passing a id token to the use case
    // When the id token is valid
    // Then should return an authenticated user object

    let test_start_timestamp = Utc::now().timestamp();

    let expected_response = VerifyIdTokenResponse {
        id: "user_id".to_string(),
        email: "user@example.com".to_string(),
        name: "User Name".to_string(),
        timestamp: Utc::now().timestamp(),
    };

    let auth_controller = setup();
    let request = VerifyIdTokenRequest {
        id_token: VALID_ID_TOKEN.to_string(),
    };

    let result = auth_controller.verify_id_token(request).await.unwrap();

    assert_eq!(result.id, expected_response.id);
    assert_eq!(result.email, expected_response.email);
    assert_eq!(result.name, expected_response.name);
    assert!(test_start_timestamp <= result.timestamp);
    assert!(result.timestamp <= Utc::now().timestamp())
}

#[actix_web::test]
async fn user_will_receive_invalid_id_token_error_when_id_token_is_invalid() {
    // Given user passing a id token to the use case
    // When the id token is invalid
    // Then should return an error

    let auth_controller = setup();
    let request = VerifyIdTokenRequest {
        id_token: INVALID_ID_TOKEN.to_string(),
    };

    let result = auth_controller.verify_id_token(request).await.unwrap_err();

    assert_eq!(result, AuthControllerError::InvalidToken);
}

#[actix_web::test]
async fn user_will_receive_invalid_input_error_when_id_token_is_empty() {
    // Given user passing an empty id token to the use case
    // When the id token is empty
    // Then should return an error

    let empty_id_token = "";
    let expected_error_message = "ID token cannot be empty";

    let auth_controller = setup();
    let request = VerifyIdTokenRequest {
        id_token: empty_id_token.to_string(),
    };

    let result = auth_controller.verify_id_token(request).await.unwrap_err();

    assert_eq!(
        result,
        AuthControllerError::InvalidInput(expected_error_message.to_string())
    );
}
