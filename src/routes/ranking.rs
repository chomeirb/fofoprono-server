use crate::routes::common::*;

#[get("/ranking")]
async fn ranking(pool: web::Data<DbPool>, user: Option<Auth<i32>>) -> Result<HttpResponse, Error> {
    let mut scores = web::block(move || {
        let conn = &mut pool.get()?;
        actions::get_user_scores(conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    if let Some(user) = user {
        let id = &user.get();
        scores.sort_by_key(|a| a.id.ne(id));
    }

    Ok(HttpResponse::Ok().json(scores.into_iter().map(UserScore::from).collect::<Vec<_>>()))
}
