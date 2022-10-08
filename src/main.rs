pub mod actions;
pub mod dbutils;
pub mod models;
pub mod routes;
pub mod schema;

use std::env;

use actix_web::{web, App, HttpServer};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(id_get_user)
            .service(username_get_user)
            .service(add_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
