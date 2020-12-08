use rocket::local::Client;
use rocket::http::{ContentType, Status};
use rocket_tut::rocket_builder;
use rocket_tut::data::db::ResponseUser;
use rocket_tut::routes::auth::Authenticated;
use serde_json;

#[test]
fn login_user_rt_test(){
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");

    // New user
    let mut new_user_response = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jerubbaal Doe",
            "email": "jerujeru@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(new_user_response.status(), Status::Ok);
    
    // Extract info on inserted user
    let response_body = new_user_response.body_string().expect("Response Body");
    let user: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    
    // Login
    let mut login_response = client.post("/api/login")
        .header(ContentType::JSON)
        .body(r##"{
            "email": "jerujeru@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(login_response.status(), Status::Ok);
    
    // Extract info on user logged in
    let response_body = login_response.body_string().expect("Response Body");
    let login_auth: Authenticated = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    
    // Test Login
    let response_cookies = login_response.cookies();
    assert_eq!(response_cookies.len(), 1);
    assert_eq!(response_cookies[0].name(), "t");
    assert_eq!(login_auth.id, user.id);
        
    // Cleanup
    if new_user_response.status() == Status::Ok {
        let res = client.delete("/api/users")
            .header(ContentType::JSON)
            .body(r##"{
                "password": "123456"
            }"##)
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
    }
}
