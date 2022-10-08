use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::models::user::{NewUser, User};

pub fn create_user(conn: &mut PgConnection, name: String, mail: String, password: String) -> User {
    use crate::schema::users;

    let new_user = NewUser {
        user_name: name,
        user_mail: mail,
        user_password: password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new post")
}
