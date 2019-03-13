use actix::{Handler, Message};
use actix_web::{AsyncResponder, Error, Json, HttpResponse, State};
use actix_web::error::ResponseError;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Local;
use crate::AppState;
use crate::api::UserCreds;
use crate::db::{DbExecutor};
use crate::db::models::{NewUser, User};
use crate::errors::*;
use diesel::prelude::*;
use futures::future::Future;

fn is_password_good(pw: &str) -> bool {
    // TODO
    true
}

fn is_email_valid(email: &str) -> bool {
    // TODO
    true
}

pub fn register_user(
    (state, user_info): (State<AppState>, Json<UserCreds>)
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    // TODO: is there a cleaner way to return with an error future?
    if !is_password_good(&user_info.password) {
        return Box::new(futures::future::err(ServiceError::WeakPassword.into()));
    }
    if !is_email_valid(&user_info.email) {
        return Box::new(futures::future::err(ServiceError::InvalidEmail.into()));
    }

    let msg = Register {
        email: user_info.email.clone(),
        password: user_info.password.clone(),
    };

    state
        .db
        .send(msg)
        .from_err()
        .and_then(|db_resp| match db_resp {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(e) => {
                log::info!("Error creating user: {}", e);
                Ok(e.error_response())
            }
        })
        .responder()
}

/// Message send to database actor to register a new user.
struct Register {
    email: String,
    password: String,
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

        let new_user = NewUser {
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
