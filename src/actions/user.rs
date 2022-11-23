use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::prelude::*;

use crate::{
    actions::common::*,
    models::{Hash, NewHash, UniqueUser, User},
    schema::hashes::dsl as hash,
    schema::users::dsl as user,
};

pub fn get_users(conn: &mut PgConnection) -> Result<Vec<User>, DbError> {
    Ok(user::users.filter(user::active.eq(true)).load(conn)?)
}

pub fn get_user(conn: &mut PgConnection, user_id: i32) -> Result<User, DbError> {
    get_row(conn, user::users, user_id)
}

pub fn name_get_user(conn: &mut PgConnection, name: String) -> Result<User, DbError> {
    Ok(user::users.filter(user::name.eq(name)).get_result(conn)?)
}

pub fn credentials_get_user(
    conn: &mut PgConnection,
    id: String,
    password: String,
) -> Result<User, DbError> {
    let user: User = user::users
        .filter(
            user::active.eq(true).and(
                user::name
                    .eq(&id)
                    .or(user::mail.eq(&id)),
            ),
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

pub fn get_and_remove_hash(conn: &mut PgConnection, hash: String) -> Result<Hash, DbError> {
    remove_row(conn, hash::hashes, hash)
}

pub fn add_hash(conn: &mut PgConnection, hash: NewHash) -> Result<Hash, DbError> {
    add_row(conn, hash::hashes, hash)
}
