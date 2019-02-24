table! {
    user (user_id) {
        user_id -> Integer,
        email -> Varchar,
        password -> Binary,
        created_at -> Datetime,
    }
}
