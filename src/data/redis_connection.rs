use std::ops::{Deref, DerefMut};
use std::env;
use dotenv::dotenv;
use r2d2;
use r2d2::PooledConnection;
use r2d2_redis::RedisConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

type Pool = r2d2::Pool<RedisConnectionManager>;
type PooledConn = PooledConnection<RedisConnectionManager>;

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
impl DerefMut for Conn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn init_pool() -> Pool {
    dotenv().ok();
    let redis_address = env::var("REDIS_ADDRESS").expect("REDIS_ADDRESS missing");
    let redis_port = env::var("REDIS_PORT").expect("REDIS_PORT missing");
    let redis_db = env::var("REDIS_DB").expect("REDIS_DB missing");
    //let redis_password = env::var("REDIS_PASSWORD").expect("REDIS_PASSWORD missing");
    let manager = RedisConnectionManager::new(format!("redis://{}:{}/{}", redis_address, redis_port, redis_db)).expect("connection manager");
    // Otherwise, with password:
    //let manager = RedisConnectionManager::new(format!("redis://user:{}@{}:{}/{}", redis_password redis_address, redis_port, redis_db)).expect("connection manager");
    match r2d2::Pool::builder().max_size(15).build(manager) {
        Ok(pool) => pool,
        Err(e) => panic!("Error: failed to create database pool {}", e),
    }
}
