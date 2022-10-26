use diesel::prelude::*;

use crate::{
    actions::common::*,
    models::{Game, UniqueProno, Prediction, Prono},
    schema::pronos::dsl as prono,
};

use super::get_games;

pub fn get_prono(conn: &mut PgConnection, user_id: i32, game_id: i32) -> Result<Prono, DbError> {
    get_row(conn, prono::pronos, (user_id, game_id))
}

fn add_prono(conn: &mut PgConnection, prono: UniqueProno) -> Result<Prono, DbError> {
    add_row(conn, prono::pronos, prono)
}

fn update_prono(conn: &mut PgConnection, prono: UniqueProno) -> Result<Prono, DbError> {
    let pred = prono.prediction;
    Ok(
        diesel::update(&get_prono(conn, prono.user_id, pred.game_id)?)
            .set((
                prono::prediction_home.eq(pred.prediction_home),
                prono::prediction_away.eq(pred.prediction_away),
            ))
            .get_result(conn)?,
    )
}

pub fn process_pronos(
    conn: &mut PgConnection,
    pronos: impl Iterator<Item = UniqueProno>,
) -> Result<Vec<Prono>, DbError> {
    pronos
        .map(|prono| update_prono(conn, prono).or_else(|_| add_prono(conn, prono)))
        .collect()
}

pub fn delete_prono(conn: &mut PgConnection, prono: UniqueProno) -> Result<Prono, DbError> {
    let pred = prono.prediction;
    Ok(diesel::delete(&get_prono(conn, prono.user_id, pred.game_id)?).get_result(conn)?)
}

pub fn delete_pronos(
    conn: &mut PgConnection,
    pronos: impl Iterator<Item = UniqueProno>,
) -> Result<Vec<Prono>, DbError> {
    pronos.map(|prono| delete_prono(conn, prono)).collect()
}

pub fn get_pronos(
    conn: &mut PgConnection,
    user_id: Option<i32>,
) -> Result<Vec<(Option<Prediction>, Game)>, DbError> {
    let games: Vec<Game> = get_games(conn)?;

    Ok(if let Some(id) = user_id {
        prono::pronos
            .filter(prono::user_id.eq(id))
            .select((
                prono::game_id,
                prono::prediction_home,
                prono::prediction_away,
            ))
            .load(conn)?
    } else {
        Vec::new()
    }
    .grouped_by(&games)
    .iter_mut()
    .map(Vec::pop)
    .zip(games)
    .collect())
}
