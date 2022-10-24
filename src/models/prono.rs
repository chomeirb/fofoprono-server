use crate::models::{Game, User};
use crate::schema::pronos;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Identifiable, Associations)]
#[diesel(belongs_to(User), belongs_to(Game))]
pub struct Prono {
    pub id: i32,

    pub user_id: i32,
    pub game_id: i32,

    pub prediction_home: i32,
    pub prediction_away: i32,
}

#[derive(Insertable, Serialize, Deserialize, Clone, Copy)]
#[diesel(table_name = pronos)]
pub struct NewProno {
    pub user_id: i32,
    #[diesel(embed)]
    pub prediction: Prediction,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Associations, Clone, Copy)]
#[diesel(table_name = pronos)]
#[diesel(belongs_to(Game))]
pub struct Prediction {
    pub game_id: i32,
    pub prediction_home: i32,
    pub prediction_away: i32,
}
