table! {
    session (session_id) {
        session_id -> Varchar,
        user_id -> Integer,
        created_at -> Datetime,
    }
}

table! {
    user (user_id) {
        user_id -> Integer,
        email -> Varchar,
        password -> Binary,
        created_at -> Datetime,
    }
}

joinable!(session -> user (user_id));

allow_tables_to_appear_in_same_query!(
    session,
    user,
);
