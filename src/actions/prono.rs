use diesel::prelude::*;

use crate::{
    actions::{common::*, game},
    models::{Game, Prono, PronoResult},
    schema::pronos::dsl as prono,
};

fn is_incoming(conn: &mut PgConnection, game_id: i32) -> Result<(), DbError> {
    if game::get_game(conn, game_id)?.time.elapsed().is_err() {
        Ok(())
    } else {
        Err(DbError::from("Cannot modify pronos for past games"))
    }
}

pub fn get_prono(conn: &mut PgConnection, user_id: i32, game_id: i32) -> Result<Prono, DbError> {
    get_row(conn, prono::pronos, (user_id, game_id))
}

fn add_prono(conn: &mut PgConnection, prono: Prono) -> Result<Prono, DbError> {
    is_incoming(conn, prono.game_id)?;
    add_row(conn, prono::pronos, prono)
}

fn update_prono(conn: &mut PgConnection, prono: Prono) -> Result<Prono, DbError> {
    is_incoming(conn, prono.game_id)?;
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
    is_incoming(conn, prono.game_id)?;
    Ok(diesel::delete(&get_prono(conn, prono.user_id, prono.game_id)?).get_result(conn)?)
}

pub fn process_pronos(
    conn: &mut PgConnection,
    pronos: impl Iterator<Item = Prono>,
) -> Result<Vec<Prono>, DbError> {
    pronos
        .map(|prono| update_prono(conn, prono.clone()).or_else(|_| add_prono(conn, prono)))
        .collect()
}

pub fn delete_pronos(
    conn: &mut PgConnection,
    pronos: impl Iterator<Item = Prono>,
) -> Result<Vec<Prono>, DbError> {
    pronos.map(|prono| delete_prono(conn, prono)).collect()
}

pub fn get_pronos(
    conn: &mut PgConnection,
    user_id: Option<i32>,
) -> Result<Vec<(Option<PronoResult>, Game)>, DbError> {
    let games: Vec<Game> = game::get_games(conn)?;

    Ok(if let Some(id) = user_id {
        prono::pronos.filter(prono::user_id.eq(id)).load(conn)?
    } else {
        Vec::new()
    }
    .grouped_by(&games)
    .iter_mut()
    .map(|prono| prono.pop().map(Prono::into))
    .zip(games)
    .collect())
}
