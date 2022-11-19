use crate::{routes::common::*, utils::mail::send_mail};

#[get("/")]
async fn index(user: Option<Auth<i32>>) -> HttpResponse {
    if let Some(user) = user {
        HttpResponse::Ok().body(format!("Hello user {}", user.get()))
    } else {
        HttpResponse::Ok().body("Hello anonymous!")
    }
}

#[get("/user")]
async fn get_user(pool: web::Data<DbPool>, user: Auth<i32>) -> Result<HttpResponse, Error> {
    let id = user.get();
    let user = web::block(move || {
        let conn = &mut pool.get()?;
        actions::get_user(conn, id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user.name))
}

#[post("/signup")]
async fn signup_process(
    pool: web::Data<DbPool>,
    user: web::Json<UniqueUser>,
) -> Result<HttpResponse, Error> {
    web::block(move || {
        let conn = &mut pool.get()?;
        let User { id, name, mail, .. } = actions::add_user(conn, user.0)?;
        let Hash { id, .. } = actions::add_hash(conn, NewHash { user_id: id })?;
        send_mail(name, mail, id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/signup/{hash}")]
async fn signup_user(
    pool: web::Data<DbPool>,
    hash: web::Path<String>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let hash = hash.into_inner();

    let User { id, .. } = web::block(move || {
        let conn = &mut pool.get()?;
        let Hash { user_id, .. } = actions::get_and_remove_hash(conn, hash)?;
        actions::verify_user(conn, user_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Auth::authenticate(&req, id)?;

    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, "/prono"))
        .finish())
}

#[delete("/user")]
async fn del_user(pool: web::Data<DbPool>, user: Auth<i32>) -> Result<HttpResponse, Error> {
    let id = user.get();
    let _user = web::block(move || {
        let conn = &mut pool.get()?;
        actions::delete_user(conn, id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/login")]
async fn login(
    pool: web::Data<DbPool>,
    user: web::Json<UniqueUser>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let User { id, .. } = web::block(move || {
        let conn = &mut pool.get()?;
        actions::credentials_get_user(conn, user.0)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Auth::authenticate(&req, id)?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/logout")]
async fn logout(user: Auth<i32>) -> Result<HttpResponse, Error> {
    user.logout();
    Ok(HttpResponse::Ok().finish())
}
