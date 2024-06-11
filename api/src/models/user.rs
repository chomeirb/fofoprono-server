use crate::models::Competition;
use crate::schema::{hashes as hashs, scores, users};

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Identifiable, Debug)]
pub struct User {
    pub id: i32,

    pub name: String,
    pub mail: String,
    pub password: String,

    pub active: bool,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct UniqueUser {
    pub name: String,
    pub mail: String,
    pub password: String,
}

#[derive(Queryable, Serialize, Deserialize, Identifiable, Associations)]
#[diesel(belongs_to(User))]
pub struct Hash {
    pub id: String,
    pub user_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = hashs)]
pub struct NewHash {
    pub user_id: i32,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize, Associations, Debug)]
#[diesel(primary_key(user_id, competition_id))]
#[diesel(belongs_to(User), belongs_to(Competition))]
pub struct Score {
    pub user_id: i32,
    pub competition_id: i32,

    pub points: i32,
    pub good: i32,
    pub perfect: i32,
}
