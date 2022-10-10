pub mod actions;
pub mod auth;
pub mod dbutils;
pub mod models;
pub mod routes;
pub mod schema;
use std::{env, pin::Pin};

use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::{
    extractors::bearer::{BearerAuth, Config},
    middleware::HttpAuthentication,
};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use routes::*;

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req
        .app_data::<Config>()
        .map(|data| Pin::new(data).get_ref().clone())
        .unwrap_or_else(Default::default);

    if let Ok(res) = auth::validate_token(credentials.token()) {
        if res {
            return Ok(req);
        }
    }

    Err((AuthenticationError::from(config).into(), req))
}

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
            .service(login)
            .service(
                web::scope("")
                    .wrap(HttpAuthentication::bearer(validator))
                    .service(get_user)
                    .service(add_user)
                    .service(del_user)
                    .service(add_prono),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
