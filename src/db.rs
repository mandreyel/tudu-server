use actix::{Actor, Handler, Message, SyncContext};
use actix_web::{Error, error::ResponseError, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{NaiveDateTime, Local};
use crate::schema::user;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::convert::From;

pub struct DbExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "user"]
pub struct User {
    pub user_id: i32,
    /// The email the user signed up with.
    pub email: String,
    /// Bcrypt hashed password, 3 + 4 + 53 bytes long, contains the salt too.
    pub password: Vec<u8>,
    /// The date on which the user account was created.
    pub created_at: NaiveDateTime,
}

#[derive(Fail, Debug)]
pub enum ServiceError {
    #[fail(display = "Invalid Credentials")]
    InternalError,
    #[fail(display = "Invalid Credentials")]
    InvalidCredentials,
    #[fail(display = "Invalid Email")]
    InvalidEmail,
    #[fail(display = "Weak Password")]
    WeakPassword,
}

impl From<Error> for ServiceError {
    fn from(e: Error) -> ServiceError {
        match e {
            _ => ServiceError::InternalError,
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalError => HttpResponse::InternalServerError().finish(),
            ServiceError::InvalidCredentials => HttpResponse::Unauthorized().finish(),
            ServiceError::InvalidEmail => HttpResponse::BadRequest().finish(),
            ServiceError::WeakPassword => HttpResponse::BadRequest().finish(),
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
        use crate::schema::user::dsl::*;

        let conn = self.0.get().unwrap();
        let hashed_pw = hash(&msg.password, DEFAULT_COST)
            .expect("Failed to hash password");
        assert!(hashed_pw.len() <= 60);

        let new_user = User {
            user_id: 0,
            email: msg.email,
            password: hashed_pw.into(),
            created_at: Local::now().naive_local(),
        };

        diesel::insert_into(user)
            .values(&new_user)
            .execute(&conn)
            .unwrap();
        let new_user: User = user
            .order(user_id.desc())
            .first(&conn)
            .unwrap();

        Ok(new_user)
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
