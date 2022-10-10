use actix_web::Error;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
}

pub fn validate_token(token: &str) -> Result<bool, Error> {
    let token = token.to_string();
    // Claims is a struct that implements Deserialize
    let token_message = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    eprintln!("token_message: {:?}", token_message);

    Ok(token_message.unwrap().claims.id == 2)
}
