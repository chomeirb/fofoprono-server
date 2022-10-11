use crate::{
    actions,
    models::{Hash, NewHash, UniqueUser, User},
    routes::common::*,
    utils::mail::send_mail,
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

#[post("/signup")]
async fn signup_process(
    pool: web::Data<DbPool>,
    user: web::Json<UniqueUser>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get().unwrap();
        let User { id, mail, .. } = actions::add_user(&mut conn, user.0).unwrap();
        let Hash { uuid, .. } = actions::add_hash(&mut conn, NewHash { id_user: id }).unwrap();
        send_mail(&mail, uuid).unwrap();
    })
    .await?;
    // .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/signup/{uuid}")]
async fn signup_user(
    pool: web::Data<DbPool>,
    uuid: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    let uuid = uuid.into_inner();

    web::block(move || {
        let mut conn = pool.get()?;
        let Hash { id_user, .. } = actions::get_and_remove_hash(&mut conn, uuid)?;
        actions::verify_user(&mut conn, id_user)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
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
    user: web::Form<UniqueUser>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let User { id, .. } = web::block(move || {
        let mut conn = pool.get()?;
        actions::credentials_get_user(&mut conn, user.0)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Identity::login(&req.extensions(), id.to_string()).unwrap();

    Ok(HttpResponse::Ok().finish())
}
