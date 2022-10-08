// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Int4,
        hometeam -> Varchar,
        awayteam -> Varchar,
        homescore -> Nullable<Int4>,
        awayscore -> Nullable<Int4>,
        time -> Int4,
        stage -> Varchar,
        homeodds -> Float8,
        awayodds -> Float8,
        drawodds -> Float8,
    }
}

diesel::table! {
    pronos (id) {
        id -> Int4,
        fk_gameid -> Int4,
        fk_userid -> Int4,
        homeprediction -> Int4,
        awayprediction -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        mail -> Varchar,
        password -> Varchar,
        score -> Int4,
        goodresult -> Int4,
        perfectresult -> Int4,
    }
}

diesel::joinable!(pronos -> games (fk_gameid));
diesel::joinable!(pronos -> users (fk_userid));

diesel::allow_tables_to_appear_in_same_query!(games, pronos, users,);
