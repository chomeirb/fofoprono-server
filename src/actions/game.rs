use diesel::prelude::*;

use crate::{actions::common::*, models::Game, schema::games::dsl as game};

pub fn get_games_ordered(conn: &mut PgConnection) -> Result<Vec<Game>, DbError> {
    Ok(game::games.order(game::time).load(conn)?)
}

pub fn get_game(conn: &mut PgConnection, id: i32) -> Result<Game, DbError> {
    get_row(conn, game::games, id)
}
