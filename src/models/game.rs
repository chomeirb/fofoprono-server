use std::time::SystemTime;

use crate::schema::{competitions, games};
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct Game {
    pub id: i32,
    pub time: SystemTime,
    pub stage: Stage,

    pub team_home: String,
    pub team_away: String,

    pub score_home: Option<i32>,
    pub score_away: Option<i32>,

    pub odds_home: Option<f64>,
    pub odds_away: Option<f64>,
    pub odds_draw: Option<f64>,

    pub competition_id: i32,

    pub external_api_id: Option<i32>,
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Debug)]
pub struct Competition {
    pub id: i32,
    pub name: String,
}

#[derive(DbEnum, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[ExistingTypePath = "crate::schema::sql_types::Stage"]
pub enum Stage {
    Group,
    Sixteen,
    Quarter,
    Semi,
    Final,
}
