use crate::{
    actions::common::*,
    models::{NewProno, Prono},
    schema::pronos::dsl::pronos,
};

pub fn add_prono(conn: &mut PgConnection, prono: NewProno) -> Result<Prono, DbError> {
    add_row(conn, pronos, prono)
}
