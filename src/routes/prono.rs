use actix_web::routes;

use crate::routes::common::*;

#[post("/prono")]
async fn add_pronos(
    pool: web::Data<DbPool>,
    user: Auth<i32>,
    req: web::Json<Vec<Prediction>>,
) -> Result<HttpResponse, Error> {
    let (user_id, predictions) = (user.get(), req.into_inner());

    let predictions = predictions
        .into_iter()
        .map(move |prediction| Prono::from((user_id, prediction)));

    let pronos = web::block(move || {
        let conn = &mut pool.get()?;
        actions::process_pronos(conn, predictions)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(pronos))
}

#[delete("/prono")]
async fn delete_pronos(
    pool: web::Data<DbPool>,
    user: Auth<i32>,
    req: web::Json<Vec<Prediction>>,
) -> Result<HttpResponse, Error> {
    let (user_id, predictions) = (user.get(), req.into_inner());

    let predictions = predictions
        .into_iter()
        .map(move |prediction| Prono::from((user_id, prediction)));

    let pronos = web::block(move || {
        let conn = &mut pool.get()?;
        actions::delete_pronos(conn, predictions)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(pronos))
}

/// Fetches games AND user pronos if authentified (otherwise pronos are null) in a tuple. Can also get any user's pronos with path
#[routes]
#[get("/prono")]
#[get("/prono/{user}")]
async fn get_games(
    pool: web::Data<DbPool>,
    user: Option<Auth<i32>>,
    path: Option<web::Path<String>>,
) -> Result<HttpResponse, Error> {
    let mut id = user.get();

    let games = web::block(move || {
        let conn = &mut pool.get()?;
        if let Some(name) = path.map(|path| path.into_inner()) {
            id = Some(actions::name_get_user(conn, name)?.id)
        }
        actions::get_pronos(conn, id)
    })
    .await?
    .map_err(error::ErrorNotFound)?;

    Ok(HttpResponse::Ok().json(games))
}
