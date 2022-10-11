use diesel::prelude::*;

use crate::{
    actions::common::*,
    models::{UniqueUser, User},
    schema::users::dsl,
};

pub fn get_user(conn: &mut PgConnection, user_id: i32) -> Result<User, DbError> {
    get_row(conn, dsl::users, user_id)
}

pub fn credentials_get_user(conn: &mut PgConnection, user: UniqueUser) -> Result<User, DbError> {
    Ok(dsl::users
        .filter(
            dsl::name
                .eq(user.name)
                .or(dsl::mail.eq(user.mail))
                .and(dsl::password.eq(user.password)),
        )
        .get_result(conn)?)
}

pub fn add_user(conn: &mut PgConnection, user: UniqueUser) -> Result<User, DbError> {
    add_row(conn, dsl::users, user)
}

pub fn delete_user(conn: &mut PgConnection, user_id: i32) -> Result<User, DbError> {
    remove_row(conn, dsl::users, user_id)
}
