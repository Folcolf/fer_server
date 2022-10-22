// @generated automatically by Diesel CLI.

diesel::table! {
    auths (user_id) {
        user_id -> Integer,
        hash -> Text,
        error -> Integer,
    }
}

diesel::table! {
    contacts (id) {
        id -> Integer,
        user_id -> Integer,
        lastname -> Text,
        firstname -> Text,
        email -> Text,
        phone -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        role -> Text,
    }
}

diesel::joinable!(auths -> users (user_id));
diesel::joinable!(contacts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(auths, contacts, users,);
