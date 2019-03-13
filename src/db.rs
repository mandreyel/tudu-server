use actix::{Actor, Handler, Message, SyncContext};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{NaiveDateTime, Local};
use crate::schema::{sessions, users};
use crate::errors::*;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

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

/// Message send to database actor to register a new user.
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
        use crate::schema::users::dsl::*;

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

        diesel::insert_into(users)
            .values(&new_user)
            .execute(&conn)
            .expect("Error creating new user");

        log::info!("Created new user");

        let new_user: User = users
            .order(user_id.desc())
            .first(&conn)
            .expect("Error fetching newly created user");
        Ok(new_user)
    }
}

/// Message send to database actor to login an existing user.
pub struct Login {
    pub email: String,
    pub password: String,
}

impl Message for Login {
    type Result = Result<Session, ServiceError>;
}

/// The object returned to the user after a successful authentication.
#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "sessions"]
pub struct Session {
    pub session_id: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

impl Handler<Login> for DbExecutor {
    type Result = <Login as Message>::Result;

    fn handle(&mut self, msg: Login, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        use crate::schema::sessions::dsl::*;
        use crate::schema::sessions::dsl::user_id as session_user_id;

        let conn = self.0.get().unwrap();
        let mut items = users
            .filter(email.eq(&msg.email))
            .load::<User>(&conn)
            .expect("Error finding user");

        if let Some(u) = items.pop() {
            let pw = msg.password.into_bytes();
            let user_pw_hash = String::from_utf8_lossy(&u.password);
            match verify(&pw, &user_pw_hash) {
                Ok(matching) => if matching {
                    // Delete old session id if it exists.
                    diesel::delete(sessions.filter(session_user_id.eq(&u.user_id)))
                        .execute(&conn)
                        .expect("Error deleting previous session");

                    // Create new session id for this user.
                    let new_session = Session {
                        session_id: Uuid::new_v4().to_simple().to_string(),
                        user_id: u.user_id,
                        created_at: Local::now().naive_local(),
                    };
                    diesel::insert_into(sessions)
                        .values(&new_session)
                        .execute(&conn)
                        .expect("Error creating new session");
                    
                    return Ok(new_session)
                },
                Err(_) => (),
            }
        }
        Err(ServiceError::InvalidCredentials)
    }
}

pub struct Logout {
    pub session: Session,
}

impl Message for Logout {
    type Result = Result<(), ServiceError>;
}

impl Handler<Logout> for DbExecutor {
    type Result = <Logout as Message>::Result;

    fn handle(&mut self, msg: Logout, _: &mut Self::Context) -> Self::Result {
        use crate::schema::sessions::dsl::*;

        let conn = self.0.get().unwrap();
        let res = diesel::delete(sessions.filter(session_id.eq(&msg.session.session_id)))
            .execute(&conn);

        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::InvalidSession),
        }
    }
}
