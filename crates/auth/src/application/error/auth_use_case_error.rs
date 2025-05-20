use crate::application::port::out::repository::auth_repository_error::AuthRepositoryError;

#[derive(Debug, PartialEq)]
pub enum AuthUseCaseError {
    RepositoryError(AuthRepositoryError),
}
