use lettre::error::Error;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub async fn send_mail(to: &str, code: i32) -> Result<(), Error> {
    let email = Message::builder()
        .from("fofoporno <fofoprono@zohomail.eu>".parse().unwrap())
        .reply_to("Yuin <fofoprono@zohomail.eu>".parse().unwrap())
        .to(["Hei <", to, ">"].join("").parse().unwrap())
        .subject("Welcome to Fofoprono!")
        .body(String::from(
            ["Your verification code:", &code.to_string()].join(" "),
        ))
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
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    };

    Ok(())
}
