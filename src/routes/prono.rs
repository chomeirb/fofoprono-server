use crate::{
    actions,
    auth::Auth,
    models::{NewProno, Prediction},
    routes::common::*,
};

#[post("/prono")]
async fn add_pronos(
    pool: web::Data<DbPool>,
    user: Auth<i32>,
    req: web::Json<Vec<Prediction>>,
) -> Result<HttpResponse, Error> {
    let (user_id, predictions) = (user.get(), req.into_inner());

    let pronos = predictions.into_iter().map(move |prediction| NewProno {
        user_id,
        prediction,
    });

    let pronos = web::block(move || {
        let conn = &mut pool.get()?;
        actions::process_pronos(conn, pronos)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(pronos))
}

#[delete("/prono")]
async fn delete_pronos(
    pool: web::Data<DbPool>,
    user: Auth<i32>,
    req: web::Json<Vec<Prediction>>,
) -> Result<HttpResponse, Error> {
    let (user_id, predictions) = (user.get(), req.into_inner());

    let pronos = predictions.into_iter().map(move |prediction| NewProno {
        user_id,
        prediction,
    });

    let pronos = web::block(move || {
        let conn = &mut pool.get()?;
        actions::delete_pronos(conn, pronos)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(pronos))
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
        actions::get_pronos(conn, id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(games))
}
