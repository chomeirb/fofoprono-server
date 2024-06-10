use actix_web::routes;

use crate::routes::common::*;

#[post("/prono")]
async fn add_pronos(
    pool: web::Data<DbPool>,
    user: Auth<i32>,
    req: web::Json<Vec<Prediction>>,
) -> Result<HttpResponse, Error> {
    let (user_id, predictions) = (user.get(), req.into_inner());

    let _pronos = web::block(move || {
        let conn = &mut pool.get()?;
        let filtered = predictions
            .into_iter()
            .map(|prediction| Prono::new(user_id, prediction))
            .filter(|prono| actions::is_incoming(conn, prono.game_id))
            .collect();
        actions::process_pronos(conn, filtered)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

#[delete("/prono")]
async fn delete_pronos(
    pool: web::Data<DbPool>,
    user: Auth<i32>,
    req: web::Json<Vec<Prediction>>,
) -> Result<HttpResponse, Error> {
    let (user_id, predictions) = (user.get(), req.into_inner());

    let _pronos = web::block(move || {
        let conn = &mut pool.get()?;
        let filtered = predictions
            .into_iter()
            .map(|prediction| Prono::new(user_id, prediction))
            .filter(|prono| actions::is_incoming(conn, prono.game_id))
            .collect();
        actions::delete_pronos(conn, filtered)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

/// Fetches games AND user pronos if authentified (otherwise pronos are null) in a tuple. Can also get any user's pronos with path.
#[routes]
#[get("/prono")]
#[get("/prono/{name}")]
async fn get_games(
    pool: web::Data<DbPool>,
    user: Option<Auth<i32>>,
    name: Option<web::Path<String>>,
    query: web::Query<CompetitionIds>,
) -> Result<HttpResponse, Error> {
    let id = user.get();
    let competition_id = query.get();
    let query_name = name.map(|path| path.into_inner());

    let games = web::block(move || {
        let conn = &mut pool.get()?;

        let id = query_name
            .map(|name| actions::name_get_user(conn, name))
            .transpose()?
            .map(|user| user.id)
            .or(id);

        actions::get_pronos(conn, id, competition_id, false)
    })
    .await?
    .map_err(error::ErrorNotFound)?;

    Ok(HttpResponse::Ok().json(
        games
            .into_iter()
            .map(|(game, prono)| {
                (
                    prono.map(|prono| {
                        serde_json::json!({
                            "prediction": {
                                "game_id": prono.game_id,
                                "prediction_home": prono.prediction_home,
                                "prediction_away": prono.prediction_away,
                            },
                            "result": prono.result
                        })
                    }),
                    game,
                )
            })
            .collect::<Vec<_>>(),
    ))
}
