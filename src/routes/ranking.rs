use crate::routes::common::*;

#[get("/ranking")]
async fn ranking(pool: web::Data<DbPool>, auth: Option<Auth<i32>>) -> Result<HttpResponse, Error> {
    let users = web::block(move || {
        let conn = &mut pool.get()?;
        actions::get_users(conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let id = auth.get();

    let scores: Vec<_> = users
        .into_iter()
        .map(|user| {
            if id == Some(user.id) {
                RankedUser::from((user, UserType::Current))
            } else {
                RankedUser::from((user, UserType::Other))
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(scores))
}
