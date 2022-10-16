use crate::schema::games;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct Game {
    pub id: i32,
    pub time: i32,
    pub stage: String,

    pub team_home: String,
    pub team_away: String,

    pub score_home: Option<i32>,
    pub score_away: Option<i32>,

    pub odds_home: f64,
    pub odds_away: f64,
    pub odds_draw: f64,
}
