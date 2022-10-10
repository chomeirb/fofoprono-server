use jsonwebtoken::{encode, EncodingKey, Header};

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

    let my_claims = Claims { id };

    // my_claims is a struct that implements Serialize
    // This will create a JWT using HS256 as algorithm
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();

    eprintln!("token: {}", token);

    Ok(HttpResponse::Ok().json(token))
}
