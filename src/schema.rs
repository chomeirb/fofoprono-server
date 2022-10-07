// @generated automatically by Diesel CLI.

diesel::table! {
    postes (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}


diesel::allow_tables_to_appear_in_same_query!(
    postes,
    posts,
);
