use diesel::prelude::*;

use crate::{actions::common::*, models::Game, schema::games::dsl as game};

pub fn get_users(conn: &mut PgConnection) -> Result<Vec<Game>, DbError> {
    get_rows(conn, game::games)
}
