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

#[derive(Serialize, Deserialize)]
pub enum UserType {
    Current,
    Other,
}

// #[derive(Serialize, Deserialize)]
// pub struct RatedUser {
//     pub name: String,
//     pub score: i32,
//     pub results_good: i32,
//     pub results_perfect: i32,
//     pub user_type: UserType,
// }

#[derive(Serialize, Deserialize)]
pub struct RankedUser {
    pub rank: i32,
    pub name: String,
    pub score: i32,
    pub results_good: i32,
    pub results_perfect: i32,
    pub user_type: UserType,
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

impl From<(i32, &User, UserType)> for RankedUser {
    fn from(
        (
            rank,
            User {
                name,
                score,
                results_good,
                results_perfect,
                ..
            },
            user_type,
        ): (i32, &User, UserType),
    ) -> Self {
        Self {
            rank,
            name: name.to_string(),
            score: *score,
            results_good: *results_good,
            results_perfect: *results_perfect,
            user_type,
        }
    }
}
