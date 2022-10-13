mod actions;
mod auth;
mod models;
mod routes;
mod schema;
mod utils;

use std::env;

use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer};
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

    let secret_key = Key::generate();

    HttpServer::new(move || {
        let session_mw =
            SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                // disable secure cookie for local testing
                .cookie_http_only(true)
                .cookie_secure(false)
                .build();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Cors::permissive())
            .wrap(session_mw)
            .service(index)
            .service(signup_process)
            .service(signup_user)
            .service(login)
            .service(get_user)
            .service(del_user)
            .service(add_prono)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
