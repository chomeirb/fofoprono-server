use crate::{routes::common::*, utils::mail::*};

use std::env;

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
        send_confirmation_mail(name, mail, id)
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

    let domain = env::var("DOMAIN").expect("DOMAIN must be set");

    Ok(HttpResponse::SeeOther()
        .append_header((header::LOCATION, format!("{}/prono", domain)))
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
    user: web::Json<(String, String)>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let User { id, .. } = web::block(move || {
        let conn = &mut pool.get()?;
        actions::credentials_get_user(conn, user.0 .0, user.0 .1)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Auth::authenticate(&req, id)?;

    let mut removal_cookie = actix_web::cookie::Cookie::build("id", "")
        .path("/")
        .http_only(true)
        .domain("api.fofoprono.chomeirb.com")
        .same_site(actix_web::cookie::SameSite::None)
        .finish();

    removal_cookie.make_removal();

    let val = header::HeaderValue::from_str(&removal_cookie.to_string()).unwrap();
    Ok(HttpResponse::Ok().append_header((crate::header::SET_COOKIE, val)).finish())
}

#[post("/logout")]
async fn logout(user: Auth<i32>) -> Result<HttpResponse, Error> {
    user.logout();
    Ok(HttpResponse::Ok().finish())
}

#[post("/contact")]
async fn contact(
    pool: web::Data<DbPool>,
    auth: Option<Auth<i32>>,
    mail: web::Json<(String, String, String)>,
) -> Result<HttpResponse, Error> {
    // If authenticated, retrieve pseudo and email from user, else use the ones provided
    let (name, email) = match auth {
        Some(auth) => {
            let id = auth.get();
            let user = web::block(move || {
                let conn = &mut pool.get()?;
                actions::get_user(conn, id)
            })
            .await?
            .map_err(error::ErrorInternalServerError)?;
            (user.name, user.mail)
        }
        None => (mail.0 .0, mail.0 .1),
    };

    // Send the mail
    send_contact_mail(name, email, mail.0 .2).map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}
