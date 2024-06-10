use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::prelude::*;

use crate::{
    actions::common::*,
    models::{Competition, Hash, NewHash, Score, UniqueUser, User},
    schema::{competitions, hashes, users},
};

#[allow(clippy::type_complexity)]
pub fn get_users_scores(
    conn: &mut PgConnection,
    competition_id: Option<i32>,
) -> Result<Vec<(User, Vec<(Score, Competition)>)>, DbError> {
    let users: Vec<User> = users::table.filter(users::active.eq(true)).load(conn)?;

    let score_query = Score::belonging_to(&users)
        .inner_join(competitions::table)
        .into_boxed();

    let score_query = match competition_id {
        Some(ids) => score_query.filter(competitions::id.eq(ids)),
        None => score_query,
    };

    let scores: Vec<Vec<(Score, Competition)>> = score_query.load(conn)?.grouped_by(&users);

    Ok(users.into_iter().zip(scores).collect::<Vec<_>>())
}

pub fn get_user(conn: &mut PgConnection, user_id: i32) -> Result<User, DbError> {
    get_row(conn, users::table, user_id)
}

pub fn name_get_user(conn: &mut PgConnection, name: String) -> Result<User, DbError> {
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
