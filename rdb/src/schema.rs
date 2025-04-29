// @generated automatically by Diesel CLI.

diesel::table! {
    orders (id) {
        id -> Varchar,
        user_id -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(orders -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    orders,
    users,
);
