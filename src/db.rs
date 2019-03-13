use actix::{Actor, SyncContext};
use chrono::NaiveDateTime;
use crate::schema::users;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct DbExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub user_id: i32,
    /// The email the user signed up with.
    pub email: String,
    /// Bcrypt hashed password, 3 + 4 + 53 bytes long, contains the salt too.
    pub password: Vec<u8>,
    /// The date on which the user account was created.
    pub created_at: NaiveDateTime,
}
