use std::{
    any::type_name,
    future::{ready, Ready},
};

use actix_session::{Session, SessionExt, SessionInsertError};
use actix_web::{error, FromRequest, HttpRequest};
use serde::{de::DeserializeOwned, Serialize};

pub struct Auth<T>(AuthInner<T>);

struct AuthInner<T> {
    data: T,
    session: Session,
}

impl<T: Serialize + DeserializeOwned> Auth<T> {
    pub fn authenticate(req: &HttpRequest, val: T) -> Result<(), SessionInsertError> {
        req.get_session().insert(T::data_key(), val)
    }

    pub fn logout(self) {
        self.0.session.purge();
    }

    /// Access the underlying data.
    pub fn get(self) -> T {
        self.0.data
    }
}

impl<T: Serialize + DeserializeOwned> AuthInner<T> {
    fn extract(req: &HttpRequest) -> Option<Self> {
        let session = req.get_session();
        let data = session.get(&T::data_key()).ok();
        data.flatten().map(|data| Self { data, session })
    }
}

impl<T> From<AuthInner<T>> for Auth<T> {
    fn from(inner: AuthInner<T>) -> Self {
        Self(inner)
    }
}

impl<T> FromRequest for Auth<T>
where
    T: Serialize + DeserializeOwned,
{
    type Error = error::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        ready(
            AuthInner::extract(req)
                .map(Self::from)
                .ok_or_else(|| error::ErrorUnauthorized("")),
        )
    }
}

pub trait OptionalAuth<T> {
    fn get(self) -> Option<T>;
}

impl<T: Serialize + DeserializeOwned> OptionalAuth<T> for Option<Auth<T>> {
    fn get(self) -> Option<T> {
        self.map(|auth| auth.0.data)
    }
}

trait AuthKey<T> {
    fn data_key() -> String;
    fn logged_in_key() -> String;
}

impl<T> AuthKey<T> for T {
    fn data_key() -> String {
        "auth.data#".to_owned() + type_name::<T>()
    }

    fn logged_in_key() -> String {
        "auth.logged_in#".to_owned() + type_name::<T>()
    }
}

// #[derive(Serialize, Deserialize)]
// struct Deadline {
//     start: i64,
//     duration: Duration,
// }

// impl Deadline {
//     fn _valid_for(duration: Duration) -> Self {
//         let start = OffsetDateTime::now_utc().unix_timestamp();
//         Self { start, duration }
//     }

//     fn _is_valid(&self) -> bool {
//         let now = OffsetDateTime::now_utc();
//         let start = OffsetDateTime::from_unix_timestamp(self.start).expect("Overflowed time...");
//         now - start < self.duration
//     }
// }
