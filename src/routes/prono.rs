use crate::{actions, models::NewProno, routes::common::*};

#[post("/prono/{id_user}/{id_game}")]
async fn add_prono(
    pool: web::Data<DbPool>,
    ids: web::Path<(i32, i32)>,
    req: web::Json<(i32, i32)>,
) -> Result<HttpResponse, Error> {
    let ((id_user, id_game), (prediction_home, prediction_away)) =
        (ids.into_inner(), req.into_inner());

    let prono = NewProno {
        id_user,
        id_game,
        prediction_home,
        prediction_away,
    };

    let user = web::block(move || {
        let mut conn = pool.get()?;
        actions::add_prono(&mut conn, prono)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}
