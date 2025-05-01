use std::io;

use actix_web::HttpServer;
use container::Container;
use opennccu_sample_backend_rust::{container, create_app::create_app};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let container = Container::new();

    HttpServer::new(move || create_app(container.clone()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
