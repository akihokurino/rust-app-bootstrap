// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
