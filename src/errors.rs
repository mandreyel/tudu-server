use actix_web::{Error, error::ResponseError, HttpResponse};
use std::convert::From;

// TODO: maybe split up errors as user facing error and internal errors that
// should not be exposed.
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
        // TODO: implement specializations.
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
