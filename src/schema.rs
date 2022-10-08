// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        user_name -> Varchar,
        user_mail -> Varchar,
        user_password -> Varchar,
    }
}
