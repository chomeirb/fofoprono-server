use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub mail: String,
    pub password: String,
    pub score: i32,
    pub goodresult: i32,
    pub perfectresult: i32,
}

use crate::schema::users;

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub mail: String,
    pub password: String,
}
