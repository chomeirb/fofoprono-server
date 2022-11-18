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

    let domain = env::var("DOMAIN").expect("DOMAIN must be set");

    let key = env::var("COOKEY").expect("COOKEY must be set");
    let secret_key = Key::from(key.as_bytes());

    let path = env::var("FRONTEND").expect("FRONTEND must be set");
    let static_files = String::from(path.strip_suffix('/').unwrap_or(&path));

    let port = env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>()
        .expect("PORT must be a number");

    HttpServer::new(move || {
        let session_mw =
            SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_http_only(true)
                .cookie_secure(true)
                .build();

        App::new()
            .wrap(Cors::default().allowed_origin(&domain))
            .app_data(web::Data::new(pool.clone()))
            .wrap(session_mw)
            .service(
                actix_web::Scope::new("/api")
                    .service(index)
                    .service(signup_process)
                    .service(signup_user)
                    .service(login)
                    .service(logout)
                    .service(get_user)
                    .service(del_user)
                    .service(add_pronos)
                    .service(delete_pronos)
                    .service(get_games)
                    .service(ranking),
            )
            .service(
                actix_files::Files::new("/", static_files.clone())
                    .index_file("index.html")
                    .default_handler(
                        actix_files::NamedFile::open(
                            vec![static_files.clone(), "index.html".to_owned()].join("/"),
                        )
                        .expect("index file should exist"),
                    ),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
