use std::ops::Deref;
use std::env;
use dotenv::dotenv;
use r2d2::PooledConnection;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

type Pool = r2d2::Pool<MongodbConnectionManager>;
type PooledConn = PooledConnection<MongodbConnectionManager>;

pub struct Conn(pub PooledConn);

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(database) => Outcome::Success(Conn(database)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
impl Deref for Conn {
    type Target = PooledConn;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn init_pool() -> Pool {
    dotenv().ok();
    let mongodb_address = env::var("MONGODB_ADDRESS").expect("MONGODB_ADDRESS missing");
    let mongodb_port = env::var("MONGODB_PORT").expect("MONGODB_PORT missing");
    let database = env::var("MONGODB_DATABASE").expect("MONGODB_DATABASE missing");
    //let mongodb_user = env::var("MONGODB_USER").expect("MONGODB_USER missing");
    //let mongodb_password = env::var("MONGODB_PASSWORD").expect("MONGODB_PASSWORD missing");
    let manager = MongodbConnectionManager::new(
        ConnectionOptions::builder()
            .with_host(&mongodb_address, mongodb_port.parse::<u16>().unwrap())
            .with_db(&database)
            //.with_auth(mongodb_user, mongodb_password)
            .build(),
    );
    match Pool::builder().max_size(64).build(manager) {
        Ok(pool) => pool,
        Err(e) => panic!("Error: failed to create database pool {}", e),
    }
}