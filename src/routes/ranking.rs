use crate::routes::common::*;

#[get("/ranking")]
async fn ranking(pool: web::Data<DbPool>, auth: Option<Auth<i32>>) -> Result<HttpResponse, Error> {
    let users = web::block(move || {
        let conn = &mut pool.get()?;
        actions::get_users(conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    let scores: Vec<_> = if let Some(auth) = auth {
        let id = auth.get();
        users
            .into_iter()
            .map(|user| {
                if user.id == id {
                    RankedUser::from((user, UserType::Current))
                } else {
                    RankedUser::from((user, UserType::Other))
                }
            })
            .collect()
    } else {
        users
            .into_iter()
            .map(|user| RankedUser::from((user, UserType::Other)))
            .collect()
    };

    Ok(HttpResponse::Ok().json(scores))
}
