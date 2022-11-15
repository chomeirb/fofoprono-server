use crate::{actions::common::*, models::Game, schema::games::dsl as game};

pub fn get_games(conn: &mut PgConnection) -> Result<Vec<Game>, DbError> {
    get_rows(conn, game::games)
}

pub fn get_game(conn: &mut PgConnection, id: i32) -> Result<Game, DbError> {
    get_row(conn, game::games, id)
}
