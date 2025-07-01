use std::sync::Arc;

use auth::{
    VerifyIdTokenUseCase, adapter::repository::auth_repository_impl::AuthRepositoryImpl,
    application::service::verify_id_token_service::VerifyIdTokenUseCaseImpl,
    framework::service::auth_firebase_service_impl::AuthFirebaseServiceImpl,
};

#[derive(Clone)]
pub struct Container {
    pub verify_id_token_use_case: Arc<dyn VerifyIdTokenUseCase>,
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Container {
    pub fn new() -> Self {
        let auth_service = Arc::new(AuthFirebaseServiceImpl::new());
        let auth_repository = Arc::new(AuthRepositoryImpl::new(auth_service.clone()));
        let verify_id_token_use_case =
            Arc::new(VerifyIdTokenUseCaseImpl::new(auth_repository.clone()));

        Self {
            verify_id_token_use_case,
        }
    }
}
