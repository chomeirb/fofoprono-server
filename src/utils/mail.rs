use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::{Message, SmtpTransport, Transport};

use crate::actions::DbError;

pub fn send_mail(to: &str, hash: String) -> Result<Response, DbError> {
    // Parse url from env file
    let domain = std::env::var("DOMAIN").expect("DOMAIN must be set");
    let mail_username = std::env::var("MAIL_USERNAME").expect("MAIL_USERNAME must be set");
    let mail_password = std::env::var("MAIL_PASSWORD").expect("MAIL_PASSWORD must be set");
    let email = Message::builder()
        .from("fofoprono <fofoprono@zohomail.eu>".parse().unwrap())
        .reply_to("Yuin <fofoprono@zohomail.eu>".parse().unwrap())
        .to(["Hei <", to, ">"].join("").parse().unwrap())
        .subject("Welcome to Fofoprono!")
        .body(
            [
                "Click this link to verify your account: https://",
                &domain,
                "/api/signup/",
                &hash,
            ]
            .join(""),
        )
        .unwrap();

    let creds = Credentials::new(mail_username, mail_password);

    let mailer = SmtpTransport::relay("smtp.zoho.eu")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    mailer.send(&email).map_err(DbError::from)
}
