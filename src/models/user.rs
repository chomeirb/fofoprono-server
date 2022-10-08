use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub user_mail: String,
    pub user_password: String,
}

use crate::schema::users;

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub user_name: String,
    pub user_mail: String,
    pub user_password: String,
}
