use crate::{
    actions::common::*,
    models::{NewUser, User},
    schema::users::dsl::users,
};

pub fn get_user(conn: &mut PgConnection, user_id: i32) -> Result<User, DbError> {
    get_row(conn, users, user_id)
}

pub fn add_user(conn: &mut PgConnection, user: NewUser) -> Result<User, DbError> {
    add_row(conn, users, user)
}

pub fn delete_user(conn: &mut PgConnection, user_id: i32) -> Result<User, DbError> {
    remove_row(conn, users, user_id)
}
