use actix_web::{HttpResponse, Responder, web};

use auth::adapter::{
    dto::VerifyIdTokenRequest, error::AuthControllerError, port::r#in::AuthController,
};

pub async fn verify_id_token(
    controller: web::Data<dyn AuthController>,
    req: web::Json<VerifyIdTokenRequest>,
) -> impl Responder {
    let result = controller.verify_id_token(req.into_inner()).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => match error {
            AuthControllerError::InvalidToken => {
                HttpResponse::Unauthorized().json("Invalid token.".to_string())
            }
            AuthControllerError::InvalidInput(error_message) => {
                HttpResponse::Forbidden().json(error_message)
            }
        },
    }
}

pub fn configure_verify_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").route("/", web::post().to(verify_id_token)));
}
