use actix_web::{AsyncResponder, Error, FutureResponse, Json, HttpRequest, HttpResponse, State};
use actix_web::error::ResponseError;
use crate::AppState;
use crate::db::{Login, Register};
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

pub fn login(
    (state, login): (State<AppState>, Json<UserInfo>)
) -> FutureResponse<HttpResponse> {
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

pub fn register_user(
    (state, user_info): (State<AppState>, Json<UserInfo>)
) -> FutureResponse<HttpResponse> {
    // TODO: verify that email and password are acceptable.
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
