use actix::{Handler, Message};
use actix_web::{AsyncResponder, Error, Json, HttpResponse, State};
use actix_web::error::ResponseError;
use crate::AppState;
use crate::api::Session;
use crate::db::DbExecutor;
use crate::errors::*;
use diesel::prelude::*;
use futures::future::Future;

pub fn logout_user(
    (state, logout): (State<AppState>, Json<Session>)
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let msg = Logout { session: logout.into_inner() };
    state
        .db
        .send(msg)
        .from_err()
        .and_then(|dp_resp| match dp_resp {
            Ok(()) => Ok(HttpResponse::Ok().finish()),
            Err(e) => Ok(e.error_response()),
        })
        .responder()
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
