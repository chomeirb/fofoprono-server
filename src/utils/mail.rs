use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::{Message, SmtpTransport, Transport};

use crate::actions::DbError;

pub fn send_mail(to: String, subject: String, body: String) -> Result<Response, DbError> {
    let smtp_host = std::env::var("SMTP_HOST").expect("SMTP_HOST must be set");
    let smtp_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

    let email = Message::builder()
        .from(
            ["Fofoprono <", &smtp_username, "@", &smtp_host, ">"]
                .join("")
                .parse()
                .unwrap(),
        )
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap();

    let creds = Credentials::new(smtp_username, smtp_password);

    let mailer = SmtpTransport::relay(&smtp_host)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    mailer.send(&email).map_err(DbError::from)
}

pub fn send_confirmation_mail(
    username: String,
    to: String,
    hash: String,
) -> Result<Response, DbError> {
    // Parse url from env file
    let api_url = std::env::var("API_URL").expect("API_URL must be set");

    let to = [&username, "<", &to, ">"].join("");
    let subject = "Bienvenue sur Fofoprono !".to_string();
    let body = [
        "Click this link to verify your account: ",
        &api_url,
        "/signup/",
        &hash,
    ]
    .join("");

    return send_mail(to, subject, body);
}

pub fn send_contact_mail(
    username: String,
    usermail: String,
    message: String,
) -> Result<Response, DbError> {
    let maintainer_mail = std::env::var("MAINTAINER_MAIL").expect("MAINTAINER_MAIL must be set");

    let to = maintainer_mail;
    let subject = "Fofoprono: new message from ".to_string() + &username;
    let body = [
        "From: ", &username, "<", &usermail, ">

        ", &message]
    .join("");

    return send_mail(to, subject, body);
}
