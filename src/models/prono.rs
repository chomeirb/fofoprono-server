use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Prono {
    pub id: i32,

    pub id_game: i32,
    pub id_user: i32,

    pub prediction_home: i32,
    pub prediction_away: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::pronos)]
pub struct NewProno {
    pub id_user: i32,
    pub id_game: i32,

    pub prediction_home: i32,
    pub prediction_away: i32,
}
