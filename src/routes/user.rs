use crate::{actions, auth::Claims, models::NewUser, routes::common::*};

use actix_web::web::ReqData;
use jsonwebtoken::{encode, get_current_timestamp, EncodingKey, Header};

#[get("/user/{id_user}")]
async fn get_user(pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let user_id = id.into_inner();

    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_user(&mut conn, user_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/user")]
async fn add_user(pool: web::Data<DbPool>, req: web::Json<NewUser>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::add_user(&mut conn, req.0)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/user")]
async fn del_user(
    pool: web::Data<DbPool>,
    user_claims: ReqData<Claims>,
) -> Result<HttpResponse, Error> {
    let user_id = user_claims.id;

    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::delete_user(&mut conn, user_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/login")]
pub async fn login(pool: web::Data<DbPool>, req: web::Json<i32>) -> Result<HttpResponse, Error> {
    let id = req.into_inner();

    web::block(move || {
        let mut conn = pool.get()?;
        actions::get_user(&mut conn, id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let claims = Claims {
        id,
        exp: get_current_timestamp() as usize + 10000,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();

    Ok(HttpResponse::Ok().json(token))
}
