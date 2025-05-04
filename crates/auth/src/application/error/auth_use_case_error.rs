use crate::application::port::repository::auth_repository_error::AuthRepositoryError;

#[derive(Debug, PartialEq)]
pub enum AuthUseCaseError {
    RepositoryError(AuthRepositoryError),
}
