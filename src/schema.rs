// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "result"))]
    pub struct Result;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "stage"))]
    pub struct Stage;
}

diesel::table! {
    competition (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Stage;

    games (id) {
        id -> Int4,
        time -> Timestamp,
        stage -> Stage,
        #[max_length = 20]
        team_home -> Varchar,
        #[max_length = 20]
        team_away -> Varchar,
        score_home -> Nullable<Int4>,
        score_away -> Nullable<Int4>,
        odds_home -> Float8,
        odds_away -> Float8,
        odds_draw -> Float8,
        competition_id -> Int4,
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
    scores (user_id, competition_id) {
        user_id -> Int4,
        competition_id -> Int4,
        points -> Int4,
        good -> Int4,
        perfect -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 20]
        name -> Varchar,
        #[max_length = 100]
        mail -> Varchar,
        #[max_length = 96]
        password -> Varchar,
        active -> Bool,
    }
}

diesel::joinable!(games -> competition (competition_id));
diesel::joinable!(hashes -> users (user_id));
diesel::joinable!(pronos -> games (game_id));
diesel::joinable!(pronos -> users (user_id));
diesel::joinable!(scores -> competition (competition_id));
diesel::joinable!(scores -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    competition,
    games,
    hashes,
    pronos,
    scores,
    users,
);
