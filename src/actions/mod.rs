mod common {
    pub use diesel::pg::PgConnection;
    use diesel::{
        associations::HasTable,
        helper_types::Find,
        query_builder::{DeleteStatement, InsertStatement, IntoUpdateTarget},
        query_dsl::{methods::FindDsl, LoadQuery},
        Insertable, RunQueryDsl, Table,
    };

    pub type DbError = Box<dyn std::error::Error + Send + Sync>;

    /// Finds a row of table T from a value of its primary key F.
    pub fn get_row<'a, T, M, F>(conn: &mut PgConnection, table: T, key: F) -> Result<M, DbError>
    where
        T: FindDsl<F>,
        Find<T, F>: LoadQuery<'a, PgConnection, M>,
    {
        Ok(table.find(key).get_result(conn)?)
    }

    /// Adds a row to table T of value M.
    pub fn add_row<'a, T, M, O>(conn: &mut PgConnection, table: T, model: M) -> Result<O, DbError>
    where
        T: Table,
        M: Insertable<T>,
        InsertStatement<T, M::Values>: LoadQuery<'a, PgConnection, O>,
    {
        Ok(diesel::insert_into(table).values(model).get_result(conn)?)
    }

    type DeleteFindStatement<F> =
        DeleteStatement<<F as HasTable>::Table, <F as IntoUpdateTarget>::WhereClause>;

    /// Deletes a row of table T from a value of its primary key F.
    pub fn remove_row<'a, T, M, F>(conn: &mut PgConnection, table: T, key: F) -> Result<M, DbError>
    where
        T: FindDsl<F>,
        Find<T, F>: IntoUpdateTarget,
        DeleteFindStatement<Find<T, F>>: LoadQuery<'a, PgConnection, M>,
    {
        Ok(diesel::delete(table.find(key)).get_result(conn)?)
    }
}

mod prono;
mod user;

pub use prono::*;
pub use user::*;
