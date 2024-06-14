use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::prelude::*;

use crate::{
    actions::common::*,
    models::{Hash, NewHash, Score, UniqueUser, User},
    schema::{hashes, scores, users},
};

pub fn get_users_score_ordered(
    conn: &mut PgConnection,
    competition_id: i32,
) -> Result<Vec<(User, Score)>, DbError> {
    Ok(users::table
        .filter(users::active.eq(true))
        .inner_join(scores::table)
        .filter(scores::competition_id.eq(competition_id))
        .order(scores::points.desc())
        .load(conn)?)
}

pub fn get_user(conn: &mut PgConnection, user_id: i32) -> Result<User, DbError> {
    get_row(conn, users::table, user_id)
}

pub fn name_get_user(conn: &mut PgConnection, name: &str) -> Result<User, DbError> {
    Ok(users::table.filter(users::name.eq(name)).get_result(conn)?)
}

pub fn credentials_get_user(
    conn: &mut PgConnection,
    id: String,
    password: String,
) -> Result<User, DbError> {
    let user: User = users::table
        .filter(
            users::active
                .eq(true)
                .and(users::name.eq(&id).or(users::mail.eq(&id))),
        )
        .get_result(conn)?;

    let parsed_hash =
        PasswordHash::new(&user.password).map_err(|err| DbError::from(err.to_string()))?;

    if Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        Ok(user)
    } else {
        Err(DbError::from("Record not found"))
    }
}

pub fn add_user(conn: &mut PgConnection, mut user: UniqueUser) -> Result<User, DbError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    user.password = argon2
        .hash_password(user.password.as_bytes(), &salt)
        .map_err(|err| DbError::from(err.to_string()))?
        .to_string();

    add_row(conn, users::table, user)
}

pub fn verify_user(conn: &mut PgConnection, user_id: i32) -> Result<User, DbError> {
    Ok(diesel::update(users::table.find(user_id))
        .set(users::active.eq(true))
        .get_result(conn)?)
}

pub fn delete_user(conn: &mut PgConnection, user_id: i32) -> Result<User, DbError> {
    remove_row(conn, users::table, user_id)
}

pub fn get_and_remove_hash(conn: &mut PgConnection, hash: String) -> Result<Hash, DbError> {
    remove_row(conn, hashes::table, hash)
}

pub fn add_hash(conn: &mut PgConnection, hash: NewHash) -> Result<Hash, DbError> {
    add_row(conn, hashes::table, hash)
}
