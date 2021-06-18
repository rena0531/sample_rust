use actix_web::{App, HttpServer};
use sample_rust::routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::routes))
        .bind("localhost:8000")?
        .run()
        .await
}