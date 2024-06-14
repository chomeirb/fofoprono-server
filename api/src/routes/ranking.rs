use crate::routes::common::*;

#[get("ranking")]
async fn ranking(
    pool: web::Data<DbPool>,
    auth: Option<Auth<i32>>,
    competition_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let conn = &mut pool.get()?;
        actions::get_users_score_ordered(conn, *competition_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let id = auth.get();

    Ok(HttpResponse::Ok().json(
        result
            .into_iter()
            .enumerate()
            .map(|(rank, (user, score))| {
                serde_json::json!({
                    "rank": rank + 1,
                    "name": user.name,
                    "connected": Some(user.id) == id,
                    "points": score.points,
                    "results_good": score.good,
                    "results_perfect": score.perfect,
                })
            })
            .collect::<Vec<_>>(),
    ))
}
