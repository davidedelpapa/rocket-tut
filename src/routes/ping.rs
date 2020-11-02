use rocket::*;

#[get("/ping")]
pub fn ping_fn() -> String {
    "PONG!".to_string()
}