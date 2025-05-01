use crate::auth::application::port::repository::auth_repository_error::AuthRepositoryError;

#[derive(Debug)]
pub enum AuthUseCaseError {
    RepositoryError(AuthRepositoryError),
}
