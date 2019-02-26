use actix_web::{AsyncResponder, Error, Json, HttpRequest, HttpResponse, State};
use actix_web::error::ResponseError;
use crate::AppState;
use crate::db::{Login, Register};
use crate::errors::*;
use futures::future::Future;

pub fn index(_req: &HttpRequest<AppState>) -> &'static str {
    "hello world"
}

/// User-provided authentication information.
#[derive(Deserialize)]
pub struct UserInfo {
    pub email: String,
    pub password: String,
}

pub fn login_user(
    (state, login): (State<AppState>, Json<UserInfo>)
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

fn is_password_good(pw: &str) -> bool {
    // TODO
    true
}

fn is_email_valid(email: &str) -> bool {
    // TODO
    true
}

pub fn register_user(
    (state, user_info): (State<AppState>, Json<UserInfo>)
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
            Err(e) => Ok(e.error_response()),
        })
        .responder()
}
