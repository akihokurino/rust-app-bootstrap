// @generated automatically by Diesel CLI.

diesel::table! {
    order_details (id) {
        id -> Varchar,
        order_id -> Varchar,
        product_name -> Varchar,
        quantity -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

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

diesel::joinable!(order_details -> orders (order_id));
diesel::joinable!(orders -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    order_details,
    orders,
    users,
);
