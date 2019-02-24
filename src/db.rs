use actix::{Actor, Handler, Message, SyncContext};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Local};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct DbExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct User {
    pub user_id: i32,
    /// The email the user signed up with.
    pub email: String,
    /// Bcrypt hashed password, 3 + 4 + 53 bytes long, contains the salt too.
    pub password: [u8; 60],
    /// The date on which the user account was created.
    pub created_at: DateTime<Local>,
}

pub enum ServiceError {
    InvalidCredentials,
}

pub struct Login {
    pub email: String,
    pub password: String,
}

impl Message for Login {
    type Result = Result<User, ServiceError>;
}

impl Handler<Login> for DbExecutor {
    type Result = Result<User, ServiceError>;

    fn handle(&mut self, msg: Login, _: &mut Self::Context) -> Self::Result {
        let hashed_pw = String::new();
        if verify(&msg.password, &hashed_pw).expect("Failed to verify password") {
            let mut buf = [0; 60];
            buf.copy_from_slice(hashed_pw.as_bytes());
            Ok(User {
                user_id: 0,
                email: msg.email,
                password: buf,
                created_at: Local::now(),
            })
        } else {
            Err(ServiceError::InvalidCredentials)
        }
    }
}

pub struct Register {
    pub email: String,
    pub password: String,
}

impl Message for Register {
    type Result = Result<User, ServiceError>;
}

impl Handler<Register> for DbExecutor {
    type Result = Result<User, ServiceError>;

    fn handle(&mut self, msg: Register, _: &mut Self::Context) -> Self::Result {
        let hashed_pw = hash(&msg.password, DEFAULT_COST).expect("Failed to hash password");
        let mut buf = [0; 60];
        buf.copy_from_slice(hashed_pw.as_bytes());
        assert!(hashed_pw.len() <= 60);
        Ok(User {
            user_id: 0,
            email: msg.email,
            password: buf,
            created_at: Local::now(),
        })
    }
}
