use actix_web::{Error, Json, HttpRequest, Responder};
use crate::AppState;

pub fn index(_req: &HttpRequest<AppState>) -> &'static str {
    "hello world"
}

/// User-provided authentication information.
#[derive(Deserialize)]
pub struct UserInfo {
    pub user: String,
    pub password: String,
}

/// The object returned to the user after a successful authentication.
#[derive(Serialize)]
pub struct Session {
    pub session_id: i32,
}

pub fn login(
    (_req, _login): (HttpRequest<AppState>, Json<UserInfo>)
) -> Result<Json<Session>, Error> {
    let session = Session { session_id: 1 };
    Ok(Json(session))
}

pub fn register_user(
    (_req, _reg_info): (HttpRequest<AppState>, Json<UserInfo>)
) -> impl Responder {
    "register"
}
