use diesel::prelude::*;

use crate::{actions::common::*, models::Game, schema::games};

pub fn get_game(conn: &mut PgConnection, id: i32) -> Result<Game, DbError> {
    get_row(conn, games::table, id)
}
