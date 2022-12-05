use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::{Message, SmtpTransport, Transport};

use crate::actions::DbError;

pub fn send_confirmation_mail(
    username: String,
    to: String,
    hash: String,
) -> Result<Response, DbError> {
    // Parse url from env file
    let api_url = std::env::var("API_URL").expect("API_URL must be set");
    let mail_username = std::env::var("MAIL_USERNAME").expect("MAIL_USERNAME must be set");
    let mail_password = std::env::var("MAIL_PASSWORD").expect("MAIL_PASSWORD must be set");

    let email = Message::builder()
        .to([&username, "<", &to, ">"].join("").parse().unwrap())
        .subject("Welcome to Fofoprono!")
        .body(
            [
                "Click this link to verify your account: ",
                &api_url,
                "/signup/",
                &hash,
            ]
            .join(""),
        )
        .unwrap();

    let creds = Credentials::new(mail_username, mail_password);

    let mailer = SmtpTransport::relay("email-smtp.eu-west-3.amazonaws.com")
        .unwrap()
        .credentials(creds)
        .port(2587)
        .build();

    // Send the email
    mailer.send(&email).map_err(DbError::from)
}

pub fn send_contact_mail(
    username: String,
    usermail: String,
    message: String,
) -> Result<Response, DbError> {
    // Parse url from env file
    let mail_username = std::env::var("MAIL_USERNAME").expect("MAIL_USERNAME must be set");
    let mail_password = std::env::var("MAIL_PASSWORD").expect("MAIL_PASSWORD must be set");
    let maintainer_mail = std::env::var("MAINTAINER_MAIL").expect("MAINTAINER_MAIL must be set");

    let email = Message::builder()
        .from(
            ["fofoprono <", &mail_username, ">"]
                .join("")
                .parse()
                .unwrap(),
        )
        .to(["Fofoprono maintainer <", &maintainer_mail, ">"]
            .join("")
            .parse()
            .unwrap())
        .subject("Contact from Fofoprono!")
        .body(
            [
                "You received a message from ",
                &username,
                " (",
                &usermail,
                "): ",
                &message,
            ]
            .join(""),
        )
        .unwrap();

    let creds = Credentials::new(mail_username, mail_password);

    let mailer = SmtpTransport::relay("email-smtp.eu-west-3.amazonaws.com")
        .unwrap()
        .credentials(creds)
        .port(2587)
        .build();

    // Send the email
    mailer.send(&email).map_err(DbError::from)
}
