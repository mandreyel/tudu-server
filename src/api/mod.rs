mod login;
mod logout;
mod index;
mod register;

pub use login::login_user;
pub use logout::logout_user;
pub use register::register_user;
pub use index::index;

use chrono::NaiveDateTime;
use crate::schema::sessions;

/// User-provided authentication information.
#[derive(Deserialize)]
pub struct UserCreds {
    pub email: String,
    pub password: String,
}

/// The object returned to the user after a successful authentication.
#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "sessions"]
pub struct Session {
    pub session_id: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}
