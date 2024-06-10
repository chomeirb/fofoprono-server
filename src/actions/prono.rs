use diesel::prelude::*;

use crate::{
    actions::{common::*, game},
    models::{Game, Prono},
    schema::{games, pronos},
};

pub fn is_incoming(conn: &mut PgConnection, game_id: i32) -> bool {
    let Ok(game) = game::get_game(conn, game_id) else {
        return false;
    };
    game.time.elapsed().is_err()
}

pub fn get_prono(conn: &mut PgConnection, user_id: i32, game_id: i32) -> Result<Prono, DbError> {
    get_row(conn, pronos::table, (user_id, game_id))
}

fn add_prono(conn: &mut PgConnection, prono: Prono) -> Result<Prono, DbError> {
    add_row(conn, pronos::table, prono)
}

fn update_prono(conn: &mut PgConnection, prono: Prono) -> Result<Prono, DbError> {
    Ok(
        diesel::update(&get_prono(conn, prono.user_id, prono.game_id)?)
            .set((
                pronos::prediction_home.eq(prono.prediction_home),
                pronos::prediction_away.eq(prono.prediction_away),
            ))
            .get_result(conn)?,
    )
}

pub fn delete_prono(conn: &mut PgConnection, prono: Prono) -> Result<Prono, DbError> {
    Ok(diesel::delete(&get_prono(conn, prono.user_id, prono.game_id)?).get_result(conn)?)
}

pub fn process_pronos(conn: &mut PgConnection, pronos: Vec<Prono>) -> Result<Vec<Prono>, DbError> {
    pronos
        .into_iter()
        .map(|prono| update_prono(conn, prono.clone()).or_else(|_| add_prono(conn, prono)))
        .collect()
}

pub fn delete_pronos(conn: &mut PgConnection, pronos: Vec<Prono>) -> Result<Vec<Prono>, DbError> {
    pronos
        .into_iter()
        .map(|prono| delete_prono(conn, prono))
        .collect()
}

pub fn get_pronos(
    conn: &mut PgConnection,
    user_id: Option<i32>,
    competition_id: Option<i32>,
    filter_incoming: bool,
) -> Result<Vec<(Game, Option<Prono>)>, DbError> {
    let user_id = user_id.unwrap_or(-999);

    let query = games::table
        .order(games::time)
        .left_outer_join(
            pronos::table.on(pronos::game_id
                .eq(games::id)
                .and(pronos::user_id.eq(user_id).or(pronos::user_id.is_null()))),
        )
        .into_boxed();

    let query = match competition_id {
        Some(competition_id) => query.filter(games::competition_id.eq(competition_id)),
        None => query,
    };

    let query = match filter_incoming {
        true => query.filter(games::time.gt(diesel::dsl::now)),
        false => query,
    };

    Ok(query.load(conn)?)
}
