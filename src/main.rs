use actix_web::{App, HttpServer};

use actix_proxy::util::google_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| App::new().configure(google_config))
    .bind("0.0.0.0:9999")?
    .run()
    .await
}
