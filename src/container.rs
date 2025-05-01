use std::sync::Arc;

use crate::auth::{
    VerifyIdTokenUseCase, adapter::repository::auth_repository_impl::AuthRepositoryImpl,
    application::use_case::verify_id_token_use_case::VerifyIdTokenUseCaseImpl,
    framework::service::auth_firebase_service_impl::AuthFirebaseServiceImpl,
};

#[derive(Clone)]
pub struct Container {
    pub verify_id_token_use_case: Arc<dyn VerifyIdTokenUseCase>,
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
