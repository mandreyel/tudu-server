use actix::{Actor, Handler, Message, SyncContext};
use actix_web::{error::ResponseError, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Local};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct DbExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    /// The email the user signed up with.
    pub email: String,
    /// Bcrypt hashed password, 3 + 4 + 53 bytes long, contains the salt too.
    pub password: String,
    /// The date on which the user account was created.
    pub created_at: DateTime<Local>,
}

#[derive(Fail, Debug)]
pub enum ServiceError {
    #[fail(display = "Invalid Credentials")]
    InvalidCredentials,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InvalidCredentials => HttpResponse::Unauthorized().finish(),
        }
    }
}

pub struct Login {
    pub email: String,
    pub password: String,
}

impl Message for Login {
    type Result = Result<Session, ServiceError>;
}

/// The object returned to the user after a successful authentication.
#[derive(Serialize)]
pub struct Session {
    pub session_id: i32,
}

impl Handler<Login> for DbExecutor {
    type Result = <Login as Message>::Result;

    fn handle(&mut self, msg: Login, _: &mut Self::Context) -> Self::Result {
        let hashed_pw = String::new();
        if verify(&msg.password, &hashed_pw).expect("Failed to verify password") {
            Ok(Session {
                session_id: 0,
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
    type Result = <Register as Message>::Result;

    fn handle(&mut self, msg: Register, _: &mut Self::Context) -> Self::Result {
        let hashed_pw = hash(&msg.password, DEFAULT_COST).expect("Failed to hash password");
        assert!(hashed_pw.len() <= 60);
        Ok(User {
            user_id: 0,
            email: msg.email,
            password: hashed_pw,
            created_at: Local::now(),
        })
    }
}
