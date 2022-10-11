use actix_web::{dev::ServiceRequest, HttpMessage};
use jsonwebtoken::{decode, errors::Error, Algorithm, DecodingKey, Validation};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: i32,
    pub exp: usize,
}

pub fn validate_token(req: &ServiceRequest, token: &str) -> Result<(), Error> {
    let token_message = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;

    req.extensions_mut().insert(token_message.claims);

    Ok(())
}
