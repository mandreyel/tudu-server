use actix::{Handler, Message};
use actix_web::{AsyncResponder, Error, Json, HttpResponse, ResponseError, State};
use bcrypt::verify;
use chrono::Local;
use crate::AppState;
use crate::api::{Session, UserCreds};
use crate::errors::*;
use crate::db::DbExecutor;
use crate::db::models::User;
use diesel::prelude::*;
use futures::future::Future;
use uuid::Uuid;

pub fn login_user(
    (state, login): (State<AppState>, Json<UserCreds>)
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let msg = Login {
        email: login.email.clone(),
        password: login.password.clone(),
    };

    state
        .db
        .send(msg)
        .from_err()
        .and_then(|dp_resp| match dp_resp {
            Ok(session) => Ok(HttpResponse::Ok().json(session)),
            Err(e) => Ok(e.error_response()),
        })
        .responder()
}

/// Message send to database actor to login an existing user.
pub struct Login {
    pub email: String,
    pub password: String,
}

impl Message for Login {
    type Result = Result<Session, ServiceError>;
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

        if let Some(user) = items.pop() {
            let pw = msg.password.into_bytes();
            let user_pw_hash = String::from_utf8_lossy(&user.password);
            match verify(&pw, &user_pw_hash) {
                Ok(matching) => if matching {
                    // Delete old session id if it exists.
                    diesel::delete(sessions.filter(session_user_id.eq(&user.user_id)))
                        .execute(&conn)
                        .expect("Error deleting previous session");

                    // Create new session id for this user.
                    let new_session = Session {
                        session_id: Uuid::new_v4().to_simple().to_string(),
                        user_id: user.user_id,
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
