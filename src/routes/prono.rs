use crate::{
    actions,
    auth::Auth,
    models::{NewProno, Prediction},
    routes::common::*,
};

#[post("/prono/{id_user}/{id_game}")]
async fn add_pronos(
    pool: web::Data<DbPool>,
    ids: web::Path<(i32, i32)>,
    req: web::Json<(i32, i32)>,
) -> Result<HttpResponse, Error> {
    let ((user_id, game_id), (prediction_home, prediction_away)) =
        (ids.into_inner(), req.into_inner());

    let prono = NewProno {
        user_id,
        prediction: Prediction {
            game_id,
            prediction_home,
            prediction_away,
        },
    };

    let user = web::block(move || {
        let conn = &mut pool.get()?;
        actions::add_prono(conn, prono)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

/// Fetches games AND user pronos if authentified (otherwise pronos are null) in a tuple.
#[get("/games")]
async fn get_games(
    pool: web::Data<DbPool>,
    user: Option<Auth<i32>>,
) -> Result<HttpResponse, Error> {
    let id = user.map(|user| user.get());

    let games = web::block(move || {
        let conn = &mut pool.get()?;
        actions::get_prono(conn, id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if id.is_some() {
        Ok(HttpResponse::Ok().json(games))
    } else {
        Ok(HttpResponse::NonAuthoritativeInformation().json(games))
    }
}
