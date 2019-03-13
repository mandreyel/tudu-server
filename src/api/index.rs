use actix_web::HttpRequest;
use crate::AppState;

pub fn index(_req: &HttpRequest<AppState>) -> &'static str {
    "hello world"
}
