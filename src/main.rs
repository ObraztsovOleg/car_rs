mod api;
mod repository;
mod models;

use actix_web::{App, HttpServer};
use api::ping::{enable};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(enable)
    })
        .bind(("localhost", 5555))?
        .run()
        .await
}
