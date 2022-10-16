use diesel::prelude::*;

use crate::{
    actions::common::*,
    models::{Game, NewProno, Prediction, Prono},
    schema::games::dsl as game,
    schema::pronos::dsl as prono,
};

pub fn add_prono(conn: &mut PgConnection, prono: NewProno) -> Result<Prono, DbError> {
    add_row(conn, prono::pronos, prono)
}

pub fn get_prono(
    conn: &mut PgConnection,
    user_id: Option<i32>,
) -> Result<Vec<(Option<Prediction>, Game)>, DbError> {
    let games: Vec<Game> = game::games.load(conn)?;

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
    .into_iter()
    .map(|preds| preds.into_iter().next())
    .zip(games)
    .collect())
}
