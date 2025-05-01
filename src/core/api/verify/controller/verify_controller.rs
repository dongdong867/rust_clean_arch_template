use actix_web::{HttpResponse, Responder, web};

use crate::{
    auth::VerifyIdTokenUseCase,
    core::api::verify::{
        error::verify_error::VerifyError, request::verify_request::VerifyRequest,
        response::verify_response::VerifyResponse,
    },
};

pub async fn verify_id_token(
    use_case: web::Data<dyn VerifyIdTokenUseCase>,
    req: web::Json<VerifyRequest>,
) -> impl Responder {
    let result = use_case
        .execute(&req.into_inner().id_token)
        .await
        .map_err(|error| VerifyError::from(error));

    match result {
        Ok(user) => HttpResponse::Ok().json(VerifyResponse::from(user)),
        Err(_) => HttpResponse::Unauthorized().json("Invalid token.".to_string()),
    }
}

pub fn configure_verify_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/verify").route("/", web::post().to(verify_id_token)));
}
