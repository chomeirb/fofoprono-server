use diesel::prelude::*;

use crate::{
    actions::common::*,
    models::{Hash, NewHash, UniqueUser, User},
    schema::hashes::dsl as hash,
    schema::users::dsl as user,
};

pub fn get_user(conn: &mut PgConnection, user_id: i32) -> Result<User, DbError> {
    get_row(conn, user::users, user_id)
}

pub fn credentials_get_user(conn: &mut PgConnection, user: UniqueUser) -> Result<User, DbError> {
    Ok(user::users
        .filter(
            user::name
                .eq(user.name)
                .or(user::mail.eq(user.mail))
                .and(user::password.eq(user.password)),
        )
        .get_result(conn)?)
}

pub fn add_user(conn: &mut PgConnection, user: UniqueUser) -> Result<User, DbError> {
    add_row(conn, user::users, user)
}

pub fn verify_user(conn: &mut PgConnection, user_id: i32) -> Result<User, DbError> {
    Ok(diesel::update(user::users.find(user_id))
        .set(user::active.eq(true))
        .get_result(conn)?)
}

pub fn delete_user(conn: &mut PgConnection, user_id: i32) -> Result<User, DbError> {
    remove_row(conn, user::users, user_id)
}

pub fn get_and_remove_hash(conn: &mut PgConnection, uuid: uuid::Uuid) -> Result<Hash, DbError> {
    remove_row(conn, hash::hashes, uuid)
}

pub fn add_hash(conn: &mut PgConnection, hash: NewHash) -> Result<Hash, DbError> {
    add_row(conn, hash::hashes, hash)
}
