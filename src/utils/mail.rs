use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::transport::smtp::Error;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_mail(to: &str, uuid: uuid::Uuid) -> Result<Response, Error> {
    let email = Message::builder()
        .from("fofoporno <fofoprono@zohomail.eu>".parse().unwrap())
        .reply_to("Yuin <fofoprono@zohomail.eu>".parse().unwrap())
        .to(["Hei <", to, ">"].join("").parse().unwrap())
        .subject("Welcome to Fofoprono!")
        .body(
            [
                "Your link: http://localhost:8080/signup/",
                &uuid.to_string(),
            ]
            .join(""),
        )
        .unwrap();

    let creds = Credentials::new(
        "fofoprono@zohomail.eu".to_string(),
        "4Tk5rjKdQKB3639".to_string(),
    );

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.zoho.eu")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    mailer.send(&email)
}
