table! {
    sessions (session_id) {
        session_id -> Varchar,
        user_id -> Integer,
        created_at -> Datetime,
    }
}

table! {
    users (user_id) {
        user_id -> Integer,
        email -> Varchar,
        password -> Binary,
        created_at -> Datetime,
    }
}

joinable!(sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
