use crate::schema::{hashes as hashs, users};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct User {
    pub id: i32,

    pub name: String,
    pub mail: String,
    pub password: String,

    pub score: i32,
    pub results_good: i32,
    pub results_perfect: i32,

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
