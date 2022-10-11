use crate::{
    actions,
    models::{UniqueUser, User},
    routes::common::*,
    utils::mail::send_mail,
};

use actix_identity::Identity;
use actix_session::Session;
use actix_web::{HttpMessage, HttpRequest};
use rand::Rng;

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
async fn pre_add_user(mail: web::Json<String>, session: Session) -> Result<HttpResponse, Error> {
    // TODO : check if mail isn't already in db
    let code = rand::thread_rng().gen_range(1000..9999);

    session.insert(code.to_string(), &mail)?;
    send_mail(&mail, code).await.ok();

    Ok(HttpResponse::Ok().finish())
}

#[post("/verif")]
async fn verif_user(code: web::Json<i32>, session: Session) -> Result<HttpResponse, Error> {
    // TODO : check if code is correct

    if session.get::<String>(&code.to_string())?.is_some() {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

#[post("/user/{code}")]
async fn add_user(
    pool: web::Data<DbPool>,
    mut user: web::Form<UniqueUser>,
    session: Session,
    code: web::Path<i32>,
) -> Result<HttpResponse, Error> {
        user.0.mail = session.get::<String>(&code.to_string())?.unwrap();

        web::block(move || {
            let mut conn = pool.get()?;
            actions::add_user(&mut conn, user.0)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

        session.purge();

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
