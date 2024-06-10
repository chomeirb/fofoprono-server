use crate::routes::common::*;

#[get("ranking")]
async fn ranking(
    pool: web::Data<DbPool>,
    auth: Option<Auth<i32>>,
    query: web::Query<CompetitionIds>,
) -> Result<HttpResponse, Error> {
    let users = web::block(move || {
        let conn = &mut pool.get()?;
        actions::get_users_scores(conn, query.competition_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let id = auth.get();

    Ok(HttpResponse::Ok().json(
        users
            .into_iter()
            .map(|(user, scores)| {
                serde_json::json!({
                    "name": user.name,
                    "connected": Some(user.id) == id,
                    "scores": scores
                        .into_iter()
                        .map(|(score, competition)| {
                            serde_json::json!({
                                "competition": competition.name,
                                "points": score.points,
                                "good": score.good,
                                "perfect": score.perfect,
                            })
                        })
                        .collect::<Vec<_>>(),
                })
            })
            .collect::<Vec<_>>(),
    ))
}
