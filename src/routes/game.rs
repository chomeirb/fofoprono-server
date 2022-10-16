use actix_web::error::ErrorInternalServerError;

use crate::{actions, routes::common::*};

#[get("/game/")]
async fn get_games(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let games = web::block(move || {
        let mut conn = pool.get()?;
        actions::get_games(&mut conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(games))
}
