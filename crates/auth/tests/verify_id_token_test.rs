use std::sync::Arc;

use auth::{
    AuthenticatedUser, VerifyIdTokenUseCase,
    adapter::{
        dto::authenticated_user_dto::AuthenticatedUserDto,
        port::service::auth_service_error::AuthServiceError,
        repository::auth_repository_impl::AuthRepositoryImpl,
    },
    application::{
        error::auth_use_case_error::AuthUseCaseError,
        port::repository::auth_repository_error::AuthRepositoryError,
        use_case::verify_id_token_use_case::VerifyIdTokenUseCaseImpl,
    },
};
use mock::mock_auth_service::MockAuthService;
use mockall::predicate::*;

mod mock;

const VALID_ID_TOKEN: &str = "valid_id_token";
const INVALID_ID_TOKEN: &str = "invalid_id_token";

fn create_mock_service() -> MockAuthService {
    let authenticated_user = AuthenticatedUserDto {
        id: "user_id".to_string(),
        email: "user@example.com".to_string(),
        name: "User Name".to_string(),
    };

    let mut mock = MockAuthService::new();
    mock.expect_verify_id_token()
        .with(eq(VALID_ID_TOKEN))
        .return_const(Ok(authenticated_user));
    mock.expect_verify_id_token()
        .with(eq(INVALID_ID_TOKEN))
        .return_const(Err(AuthServiceError::InvalidCredentials));

    return mock;
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
        .execute(VALID_ID_TOKEN)
        .await
        .unwrap();

    assert_eq!(result, expected_user);
}

#[actix_web::test]
async fn user_will_received_error_when_id_token_is_invalid() {
    // Given user passing a id token to the use case
    // When the id token is invalid
    // Then should return an error

    let verify_id_token_use_case = setup();
    let result = verify_id_token_use_case
        .execute(INVALID_ID_TOKEN)
        .await
        .unwrap_err();

    assert_eq!(
        result,
        AuthUseCaseError::RepositoryError(AuthRepositoryError::InvalidIdToken)
    );
}
