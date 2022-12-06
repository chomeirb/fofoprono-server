use diesel::prelude::*;

use crate::{
    actions::{common::*, game},
    models::{Game, PredictionResult, Prono, PronoResult},
    schema::pronos::dsl as prono,
};

pub fn is_incoming(conn: &mut PgConnection, game_id: i32) -> bool {
    let Ok(game) = game::get_game(conn, game_id) else {
        return false;
    };
    game.time.elapsed().is_err()
}

pub fn get_prono(conn: &mut PgConnection, user_id: i32, game_id: i32) -> Result<Prono, DbError> {
    get_row(conn, prono::pronos, (user_id, game_id))
}

fn add_prono(conn: &mut PgConnection, prono: Prono) -> Result<Prono, DbError> {
    add_row(conn, prono::pronos, prono)
}

fn update_prono(conn: &mut PgConnection, prono: Prono) -> Result<Prono, DbError> {
    Ok(
        diesel::update(&get_prono(conn, prono.user_id, prono.game_id)?)
            .set((
                prono::prediction_home.eq(prono.prediction_home),
                prono::prediction_away.eq(prono.prediction_away),
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
) -> Result<Vec<(PronoResult, Game)>, DbError> {
    let games: Vec<Game> = game::get_games_ordered(conn)?;

    Ok(if let Some(id) = user_id {
        prono::pronos.filter(prono::user_id.eq(id)).load(conn)?
    } else {
        Vec::new()
    }
    .grouped_by(&games)
    .iter_mut()
    .map(|prono| prono.pop().map(Prono::into))
    .zip(games)
    .map(|(prono, game)| {
        (
            prono.unwrap_or_else(|| PronoResult {
                prediction: None,
                result: if user_id.is_some() && !is_incoming(conn, game.id) {
                    Some(PredictionResult::Wrong)
                } else {
                    None
                },
            }),
            game,
        )
    })
    .collect())
}
