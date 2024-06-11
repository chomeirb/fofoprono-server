mod common {
    pub(crate) use crate::{actions, auth::*, models::*};
    pub use actix_web::{
        delete, error, get,
        http::header::{self, Header},
        post, web, Error, HttpRequest, HttpResponse,
    };

    use diesel::r2d2;
    pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<diesel::PgConnection>>;
}

mod prono;
mod ranking;
mod user;

pub use prono::*;
pub use ranking::*;
pub use user::*;
