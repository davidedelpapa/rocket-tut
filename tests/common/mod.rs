use crate::lazy_static::lazy_static;

use rocket::local::Client;
use rocket_tut::rocket_builder;

pub fn setup () -> &'static Client {
    lazy_static! {
        static ref CLIENT: Client = Client::new(rocket_builder()).expect("Valid Rocket instance");
    }
    &*CLIENT
}
