use crate::actions;
use crate::models::NewUser;

use actix_web::{get, post, web, Error, HttpResponse};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/user/{user_id}/id")]
pub async fn id_get_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();

    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::find_user_id(&mut conn, user_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/user/{user_name}/name")]
pub async fn username_get_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();

    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::find_user_username(&mut conn, user_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/user")]
pub async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<NewUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::insert_user(&mut conn, &form.0)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}
