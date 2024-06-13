use crate::models::{Game, User};
use crate::schema::pronos;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(DbEnum, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[ExistingTypePath = "crate::schema::sql_types::Result"]
pub enum PredictionResult {
    Exact,
    Correct,
    Wrong,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Identifiable, Associations, Clone)]
#[diesel(belongs_to(User), belongs_to(Game), primary_key(user_id, game_id))]
pub struct Prono {
    pub user_id: i32,
    pub game_id: i32,

    pub prediction_home: i32,
    pub prediction_away: i32,

    pub result: Option<PredictionResult>,
}

// Not able to have embedded foreign key with belongs_to associations: duplicate data
#[derive(Insertable, Serialize, Deserialize, Clone, Copy)]
#[diesel(table_name = pronos)]
pub struct Prediction {
    pub game_id: i32,
    #[diesel(column_name = prediction_home)]
    pub home: i32,
    #[diesel(column_name = prediction_away)]
    pub away: i32,
}

impl Prono {
    pub fn new(user_id: i32, prediction: Prediction) -> Self {
        Self {
            user_id,
            game_id: prediction.game_id,
            prediction_home: prediction.home,
            prediction_away: prediction.away,
            result: None,
        }
    }
}
