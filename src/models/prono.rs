use crate::models::{Game, User};
use crate::schema::pronos;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Identifiable, Associations)]
#[diesel(belongs_to(User), belongs_to(Game), primary_key(user_id, game_id))]
pub struct Prono {
    pub user_id: i32,
    pub game_id: i32,

    pub prediction_home: i32,
    pub prediction_away: i32,

    pub result: String,
}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = pronos)]
pub struct UniqueProno {
    pub user_id: i32,
    #[diesel(embed)]
    pub prediction: Prediction,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Associations, Clone)]
#[diesel(table_name = pronos)]
#[diesel(belongs_to(Game))]
pub struct Prediction {
    pub game_id: i32,
    pub prediction_home: i32,
    pub prediction_away: i32,
    pub result: String,
}
