use crate::models::{Game, User};
use crate::schema::pronos;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, DbEnum, Clone)]
#[DieselTypePath = "crate::schema::sql_types::Result"]
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

#[derive(Queryable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = pronos)]
pub struct PronoResult {
    #[diesel(embed)]
    pub prediction: Prediction,
    pub result: Option<PredictionResult>,
}

// Not able to have embedded foreign key with belongs_to associations: duplicate data
#[derive(Insertable, Serialize, Deserialize, Clone, Copy)]
#[diesel(table_name = pronos)]
pub struct Prediction {
    pub game_id: i32,
    pub prediction_home: i32,
    pub prediction_away: i32,
}

impl From<Prono> for PronoResult {
    fn from(
        Prono {
            game_id,
            prediction_home,
            prediction_away,
            result,
            ..
        }: Prono,
    ) -> Self {
        Self {
            prediction: Prediction {
                game_id,
                prediction_home,
                prediction_away,
            },
            result,
        }
    }
}

impl From<(i32, Prediction)> for Prono {
    fn from(
        (
            user_id,
            Prediction {
                game_id,
                prediction_home,
                prediction_away,
            },
        ): (i32, Prediction),
    ) -> Self {
        Self {
            user_id,
            game_id,
            prediction_home,
            prediction_away,
            result: None,
        }
    }
}
