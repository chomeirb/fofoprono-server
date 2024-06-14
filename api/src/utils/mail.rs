use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::{Message, SmtpTransport, Transport};

use crate::actions::DbError;

pub fn send_mail(
    to: &str,
    subject: &str,
    body: String,
    reply_to: &str,
) -> Result<Response, DbError> {
    let host = std::env::var("MAIL_HOST").expect("MAIL_HOST must be set");
    let user = std::env::var("MAIL_USER").expect("MAIL_USER must be set");
    let password = std::env::var("MAIL_PASSWORD").expect("MAIL_PASSWORD must be set");

    let email = Message::builder()
        .from(["Fofoprono <", reply_to, ">"].join("").parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap();

    let creds = Credentials::new(user, password);

    let mailer = SmtpTransport::relay(&host)
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email).map_err(DbError::from)
}

pub fn send_confirmation_mail(
    username: String,
    to: String,
    hash: String,
) -> Result<Response, DbError> {
    let url = std::env::var("API_URL").expect("API_URL must be set");
    let maintainer_mail = std::env::var("MAINTAINER_MAIL").expect("MAINTAINER_MAIL must be set");

    let to = [&username, "<", &to, ">"].join("");
    let subject = "Bienvenue sur Fofoprono !".to_string();
    let body = [
        "Click this link to verify your account: ",
        &url,
        "/signup/",
        &hash,
    ]
    .join("");

    send_mail(&to, &subject, body, &maintainer_mail)
}

pub fn send_contact_mail(
    username: String,
    usermail: String,
    message: String,
) -> Result<Response, DbError> {
    let maintainer_mail = std::env::var("MAINTAINER_MAIL").expect("MAINTAINER_MAIL must be set");

    let to = &maintainer_mail;
    let subject = "Fofoprono: new message from ".to_string() + &username;
    let body = [
        "From: ",
        &username,
        "<",
        &usermail,
        ">

        ",
        &message,
    ]
    .join("");

    send_mail(to, &subject, body, &maintainer_mail)
}
