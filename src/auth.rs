use std::{
    any::type_name,
    future::{ready, Ready},
};

use actix_session::{SessionExt, SessionInsertError};
use actix_web::{error, FromRequest};
use serde::{de::DeserializeOwned, Serialize};

pub struct Auth<T>(pub T);

impl<T: Serialize + DeserializeOwned> Auth<T> {
    pub fn authenticate(req: &actix_web::HttpRequest, val: T) -> Result<(), SessionInsertError> {
        req.get_session().insert(Self::key(), val)
    }

    fn extract(req: &actix_web::HttpRequest) -> Option<Self> {
        let value = req.get_session().get::<T>(&Self::key()).ok();
        value.flatten().map(T::into)
    }

    fn key() -> String {
        "Auth".to_owned() + type_name::<T>()
    }
}

impl<T> FromRequest for Auth<T>
where
    T: Serialize + DeserializeOwned,
{
    type Error = error::Error;

    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        ready(Self::extract(req).ok_or_else(|| error::ErrorUnauthorized("")))
    }
}

impl<T> From<T> for Auth<T> {
    fn from(val: T) -> Self {
        Self(val)
    }
}

impl<T> std::ops::Deref for Auth<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Auth<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
