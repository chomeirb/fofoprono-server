use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation, errors::Error};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub exp: usize,
}

pub fn validate_token(token: &str) -> Result<(), Error> {
    let token = token;

    let token_message = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;

    eprintln!("token_message: {:?}", token_message);

    Ok(())
}
