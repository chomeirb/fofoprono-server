use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::{Message, SmtpTransport, Transport};

use crate::actions::DbError;

pub fn send_mail(to: &str, hash: String) -> Result<Response, DbError> {
    let email = Message::builder()
        .from("fofoporno <fofoprono@zohomail.eu>".parse().unwrap())
        .reply_to("Yuin <fofoprono@zohomail.eu>".parse().unwrap())
        .to(["Hei <", to, ">"].join("").parse().unwrap())
        .subject("Welcome to Fofoprono!")
        .body(
            [
                "Click this link to verify your account: http://localhost:8080/api/signup/",
                &hash,
            ]
            .join(""),
        )
        .unwrap();

    let creds = Credentials::new(
        "fofoprono@zohomail.eu".to_string(),
        "4Tk5rjKdQKB3639".to_string(),
    );

    let mailer = SmtpTransport::relay("smtp.zoho.eu")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    mailer.send(&email).map_err(DbError::from)
}
