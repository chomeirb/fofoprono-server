use crate::actions::user::create_user;
use crate::dbutils::establish_connection;
use crate::models::user::NewUser;

use actix_web::{get, post, HttpResponse, Responder};

#[get("/user")]
pub async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/user")]
pub async fn set_user(req_body: String) -> impl Responder {
    //convert req_body to json
    let json: NewUser = serde_json::from_str(&req_body).unwrap();

    let connection = &mut establish_connection();

    create_user(
        connection,
        json.user_name,
        json.user_mail,
        json.user_password,
    );

    HttpResponse::Ok().body(req_body)
}
