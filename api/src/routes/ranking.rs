use crate::routes::common::*;

#[get("/ranking")]
async fn ranking(pool: web::Data<DbPool>, auth: Option<Auth<i32>>) -> Result<HttpResponse, Error> {
    let users = web::block(move || {
        let conn = &mut pool.get()?;
        actions::get_users_ordered(conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    let id = auth.get();

    let mut rank = 1;
    let scores: Vec<_> = users
        .iter()
        .enumerate()
        .map(|(i, user)| {
            if i > 0 && Some(true) != users.get(i - 1).map(|user2| user2.score == user.score) {
                rank = i as i32 + 1;
            }

            if id == Some(user.id) {
                RankedUser::from((rank, user, UserType::Current))
            } else {
                RankedUser::from((rank, user, UserType::Other))
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(scores))
}
