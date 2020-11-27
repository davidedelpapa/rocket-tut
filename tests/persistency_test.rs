use rocket::local::Client;
use rocket_tut::rocket_builder;
use rocket::http::{ContentType, Status};
use rocket_tut::data::db::ResponseUser;
use serde_json;

#[test]
fn create_and_persist_test(){
    // We make sure that client1 gets properly disposed of
    {
        let client1 = Client::new(rocket_builder()).expect("Valid Rocket instance");
        let mut response = client1.post("/api/users")
            .header(ContentType::JSON)
            .body(r##"{
                "name": "John J.Doe",
                "email": "jjdd@m.com",
                "password": "123456"
            }"##)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        let response_body = response.body_string().expect("Response Body");
        let user: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
        assert_eq!(user.name, "John J.Doe");
        assert_eq!(user.email, "jjdd@m.com");
    }

    // Let's create a new client and ask for info there using the email
    let client2 = Client::new(rocket_builder()).expect("Valid Rocket instance");
    let mut response = client2.get(format!("/api/users/{}", "jjdd@m.com")).dispatch();
    let response_body = response.body_string().expect("Response Body");
    let user: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(user.name, "John J.Doe");
    assert_eq!(user.email, "jjdd@m.com");

    // Cleanup
    if response.status() == Status::Ok {
        let res = client2.delete(format!("/api/users/{}", user.id))
            .header(ContentType::JSON)
            .body(r##"{
                "password": "123456"
            }"##)
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
    }
}