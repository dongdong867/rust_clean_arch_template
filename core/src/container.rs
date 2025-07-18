use std::sync::Arc;

use auth::adapter::controller::AuthControllerImpl;
use auth::adapter::port::r#in::AuthController;
use auth::adapter::repository::AuthRepositoryImpl;
use auth::application::service::VerifyIdTokenUseCaseImpl;
use auth::framework::external::AuthFirebaseServiceImpl;

#[derive(Clone)]
pub struct Container {
    pub auth_controller: Arc<dyn AuthController>,
}

impl Default for Container {
    fn default() -> Self { Self::new() }
}

impl Container {
    pub fn new() -> Self {
        let auth_service = Arc::new(AuthFirebaseServiceImpl::new());
        let auth_repository = Arc::new(AuthRepositoryImpl::new(auth_service.clone()));
        let verify_id_token_use_case =
            Arc::new(VerifyIdTokenUseCaseImpl::new(auth_repository.clone()));
        let auth_controller = Arc::new(AuthControllerImpl::new(verify_id_token_use_case.clone()));

        Self { auth_controller }
    }
}
