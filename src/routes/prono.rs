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
            .map(|prediction| Prono::from((user_id, prediction)))
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
            .map(|prediction| Prono::from((user_id, prediction)))
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
) -> Result<HttpResponse, Error> {
    let id = user.get();

    let games = web::block(move || {
        let conn = &mut pool.get()?;

        let Some(name) = name.map(|path| path.into_inner()) else {
            return actions::get_pronos(conn, id);
        };

        let id = Some(actions::name_get_user(conn, name)?.id);
        actions::get_pronos(conn, id).map(|games| {
            games
                .into_iter()
                .filter(|prono_game| prono_game.1.time.elapsed().is_ok())
                .collect()
        })
    })
    .await?
    .map_err(error::ErrorNotFound)?;

    Ok(HttpResponse::Ok().json(games))
}
