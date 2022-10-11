// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Int4,
        time -> Int4,
        stage -> Varchar,
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
        id -> Uuid,
        id_user -> Int4,
    }
}

diesel::table! {
    pronos (id) {
        id -> Int4,
        id_user -> Int4,
        id_game -> Int4,
        prediction_home -> Int4,
        prediction_away -> Int4,
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

diesel::joinable!(hashes -> users (id_user));
diesel::joinable!(pronos -> games (id_game));
diesel::joinable!(pronos -> users (id_user));

diesel::allow_tables_to_appear_in_same_query!(
    games,
    hashes,
    pronos,
    users,
);
