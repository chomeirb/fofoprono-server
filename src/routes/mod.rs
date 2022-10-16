mod common {
    pub use actix_web::{delete, get, post, web, Error, HttpResponse};
    use diesel::r2d2;

    pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<diesel::PgConnection>>;
}

mod game;
mod prono;
mod user;

pub use prono::*;
pub use user::*;
