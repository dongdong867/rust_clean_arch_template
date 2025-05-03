use actix_web::{
    App, Error,
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
};

use crate::{api::configure_verify_routes, container::Container};

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
        .app_data(container.verify_id_token_use_case.clone())
        .configure(configure_verify_routes)
}
