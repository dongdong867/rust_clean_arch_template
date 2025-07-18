use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::{App, Error};

use crate::api::route::configure_verify_routes;
use crate::container::Container;

pub fn create_app(
    container: Container,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    App::new()
        .app_data(container.auth_controller.clone())
        .configure(configure_verify_routes)
}
