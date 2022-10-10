use jsonwebtoken::{encode, EncodingKey, Header, get_current_timestamp};

use crate::{actions, auth::Claims, routes::common::*};

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
