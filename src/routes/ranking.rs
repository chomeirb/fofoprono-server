use crate::routes::common::*;

#[get("/ranking")]
async fn ranking(pool: web::Data<DbPool>, auth: Option<Auth<i32>>) -> Result<HttpResponse, Error> {
    let users = web::block(move || {
        let conn = &mut pool.get()?;
        actions::get_users(conn)
    })
    .await?
    .map_err(ErrorInternalServerError)?;

    // let id = if let Some(auth) = auth {
    //     auth.get()
    // } else {
    //     1
    // };

    let scores: Vec<_> = if let Some(auth) = auth {
        let id = auth.get();
        users
            .into_iter()
            .map(|user| {
                if user.id == id {
                    UserType::Current(UserScore::from(user))
                } else {
                    UserType::Other(UserScore::from(user))
                }
            })
            .collect()
    } else {
        users
            .into_iter()
            .map(|user| UserType::Other(UserScore::from(user)))
            .collect()
    };

    // if let Some(user) = user {
    //     let id = &user.get();
    //     scores.sort_by_key(|a| a.id.ne(id));
    // }

    Ok(HttpResponse::Ok().json(scores))
}
