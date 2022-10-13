use crate::{
    actions,
    auth::Auth,
    models::{Hash, NewHash, UniqueUser, User},
    routes::common::*,
    utils::mail::send_mail,
};

use actix_web::{error::ErrorInternalServerError, http::header, HttpRequest};

#[get("/")]
async fn index(user: Option<Auth<i32>>) -> HttpResponse {
    if let Some(user) = user {
        HttpResponse::Ok().body(format!("Hello user {}", user.0))
    } else {
        HttpResponse::Ok().body("Hello anonymous!")
    }
}

#[get("/user")]
async fn get_user(pool: web::Data<DbPool>, user: Auth<i32>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_user(&mut conn, user.0)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/signup")]
async fn signup_process(
    pool: web::Data<DbPool>,
    user: web::Json<UniqueUser>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let mut conn = pool.get()?;
        let User { id, mail, .. } = actions::add_user(&mut conn, user.0)?;
        let Hash { id, .. } = actions::add_hash(&mut conn, NewHash { id_user: id })?;
        send_mail(&mail, id)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/signup/{uuid}")]
async fn signup_user(
    pool: web::Data<DbPool>,
    uuid: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let uuid = uuid.into_inner();

    let User { id, .. } = web::block(move || {
        let mut conn = pool.get()?;
        let Hash { id_user, .. } = actions::get_and_remove_hash(&mut conn, uuid)?;
        actions::verify_user(&mut conn, id_user)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Auth::authenticate(&req, id)?;

    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/"))
        .finish())
}

#[delete("/user")]
async fn del_user(pool: web::Data<DbPool>, user: Auth<i32>) -> Result<HttpResponse, Error> {
    let user_id = user.0;

    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::delete_user(&mut conn, user_id)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/login")]
async fn login(
    pool: web::Data<DbPool>,
    user: web::Json<UniqueUser>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let User { id, .. } = web::block(move || {
        let mut conn = pool.get()?;
        actions::credentials_get_user(&mut conn, user.0)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Auth::authenticate(&req, id)?;

    Ok(HttpResponse::Ok()
        .append_header((header::LOCATION, "/"))
        .finish())
}
