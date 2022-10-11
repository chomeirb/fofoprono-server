use crate::{
    actions,
    mail::send_mail,
    models::{UniqueUser, User},
    routes::common::*,
};

use actix_identity::Identity;
use actix_web::{HttpMessage, HttpRequest};

#[get("/user")]
async fn get_user(pool: web::Data<DbPool>, user: Identity) -> Result<HttpResponse, Error> {
    let user_id = user.id().unwrap().parse().unwrap();

    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_user(&mut conn, user_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/user")]
async fn add_user(
    pool: web::Data<DbPool>,
    user: web::Form<UniqueUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::add_user(&mut conn, user.0)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    send_mail(&user.mail).await.ok();

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/user")]
async fn del_user(pool: web::Data<DbPool>, user: Identity) -> Result<HttpResponse, Error> {
    let user_id = user.id().unwrap().parse().unwrap();

    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::delete_user(&mut conn, user_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/login")]
async fn login(
    pool: web::Data<DbPool>,
    request: HttpRequest,
    user: web::Form<UniqueUser>,
) -> Result<HttpResponse, Error> {
    let User { id, .. } = web::block(move || {
        let mut conn = pool.get()?;
        actions::credentials_get_user(&mut conn, user.0)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Identity::login(&request.extensions(), id.to_string()).unwrap();

    Ok(HttpResponse::Ok().finish())
}
