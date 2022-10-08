use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::{NewUser, User};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn insert_user(conn: &mut PgConnection, new_user: &NewUser) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    let user: User = diesel::insert_into(users)
        .values(new_user)
        .get_result(conn)?;

    Ok(user)
}

pub fn find_user_id(conn: &mut PgConnection, user_id: i32) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(user_id))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub fn find_user_username(
    conn: &mut PgConnection,
    user_name: String,
) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(username.eq(user_name))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

// Generic find_user::<column>?