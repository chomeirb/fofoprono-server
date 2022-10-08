pub mod actions;
pub mod dbutils;
pub mod models;
pub mod routes;
pub mod schema;

use actix_web::{App, HttpServer};
use routes::user::{get_user, set_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_user).service(set_user))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
