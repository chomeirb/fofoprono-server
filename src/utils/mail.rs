use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::{Message, SmtpTransport, Transport};

use crate::actions::DbError;

pub fn send_mail(username: String, to: String, hash: String) -> Result<Response, DbError> {
    // Parse url from env file
    let api_url = std::env::var("API_URL").expect("API_URL must be set");
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

    let creds = Credentials::new(smtp_username, smtp_password);

    let mailer = SmtpTransport::relay(&smtp_host)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    mailer.send(&email).map_err(DbError::from)
}
