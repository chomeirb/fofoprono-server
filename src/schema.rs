// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "result"))]
    pub struct Result;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "stage"))]
    pub struct Stage;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Stage;

    games (id) {
        id -> Int4,
        time -> Timestamp,
        stage -> Stage,
        team_home -> Varchar,
        team_away -> Varchar,
        score_home -> Nullable<Int4>,
        score_away -> Nullable<Int4>,
        odds_home -> Float8,
        odds_away -> Float8,
        odds_draw -> Float8,
    }
}

diesel::table! {
    hashes (id) {
        id -> Text,
        user_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Result;

    pronos (user_id, game_id) {
        user_id -> Int4,
        game_id -> Int4,
        prediction_home -> Int4,
        prediction_away -> Int4,
        result -> Nullable<Result>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        mail -> Varchar,
        password -> Varchar,
        score -> Int4,
        results_good -> Int4,
        results_perfect -> Int4,
        active -> Bool,
    }
}

diesel::joinable!(hashes -> users (user_id));
diesel::joinable!(pronos -> games (game_id));
diesel::joinable!(pronos -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    games,
    hashes,
    pronos,
    users,
);
