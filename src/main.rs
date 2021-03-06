#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate failure;

mod api;
mod db;
mod errors;
mod schema;

use actix::prelude::*;
use actix_web::{http, server, App};
use actix_web::middleware::session::{CookieSessionBackend, SessionStorage};
use actix_web::middleware::Logger;
use crate::db::DbExecutor;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use simplelog::{Config, LevelFilter, TermLogger};

// TODO
static SESSION_SIGNING_KEY: &[u8] = &[0; 32];

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

fn main() {
    TermLogger::init(LevelFilter::Info, Config::default()).unwrap();
    let sys = System::new("tudu-server");

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool");
    let db: Addr<DbExecutor> = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

    let app = move || {
        let session_store = SessionStorage::new(
            // TODO: remove the Secure: false flag in production.
            CookieSessionBackend::signed(SESSION_SIGNING_KEY).secure(false)
        );
        App::with_state(AppState { db: db.clone() })
            .middleware(Logger::default())
            .middleware(session_store)
            .route("/user/login", http::Method::POST, api::login_user)
            .route("/user/logout", http::Method::POST, api::logout_user)
            .route("/user/create", http::Method::POST, api::register_user)
    };

    server::new(app).bind("localhost:8888").unwrap().start();

    sys.run();
}
