#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] use rocket::*;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::helmet::SpaceHelmet;

#[get("/echo/<echo>")]
fn echo_fn(echo: String) -> String {
    format!("{}", echo)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().attach(SpaceHelmet::default())
    .mount("/", routes![echo_fn])
    .mount("/files", StaticFiles::from("static/"))
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn echo_test() {
        let client = Client::new(rocket()).expect("Valid Rocket instance");
        let mut response = client.get("/echo/test_echo").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("test_echo".into()));
    }
}