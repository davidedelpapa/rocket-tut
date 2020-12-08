use rocket::local::Client;
use rocket::http::{ContentType, Status};
use rocket_tut::rocket_builder;
use rocket_tut::data::db::ResponseUser;
use rocket_tut::routes::auth::Authenticated;
use serde_json;

#[test]
fn info_user_rt_privacy_fail(){
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");

    // New user 1
    let mut new_user_response = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jessie Doe",
            "email": "jessied@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(new_user_response.status(), Status::Ok);

    // Extract info on inserted user
    let response_body = new_user_response.body_string().expect("Response Body");
    let user1: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    
    // New user 2
    let mut new_user_response = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jacqueline Doe",
            "email": "jacqd@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(new_user_response.status(), Status::Ok);
    
    // Extract info on inserted user
    let response_body = new_user_response.body_string().expect("Response Body");
    let user2: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    
    // Login user 1
    let mut login_response = client.post("/api/login")
        .header(ContentType::JSON)
        .body(r##"{
            "email": "jessied@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(login_response.status(), Status::Ok);
    
    // Extract info on user logged in
    let response_body = login_response.body_string().expect("Response Body");
    let login_auth: Authenticated = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    
    // Test Login 1
    let response_cookies = login_response.cookies();
    assert_eq!(response_cookies[0].name(), "t");
    assert_eq!(login_auth.id, user1.id);

    // Info on User 2
    let mut info_response = client.get(format!("/api/users/{}", user2.id)).dispatch();
    assert_eq!(info_response.status(), Status::Ok);
    let response_body = info_response.body_string().expect("Response Body");

    // Test if it is scoped
    assert_eq!(response_body.contains("\"name\":"), true);
    assert_eq!(response_body.contains("\"email\":"), false);

    // Cleanup User 1
    let res = client.delete("/api/users/")
        .header(ContentType::JSON)
        .body(r##"{
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    
    // Cleanup User 2 (login first)
    let login_response = client.post("/api/login")
        .header(ContentType::JSON)
        .body(r##"{
            "email": "jacqd@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(login_response.status(), Status::Ok);
    let res = client.delete("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
}

#[test]
fn id_user_rt_privacy_fail(){
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");

    // New user 1
    let mut new_user_response = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "juvenal Doe",
            "email": "juvied@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(new_user_response.status(), Status::Ok);

    // Extract info on inserted user
    let response_body = new_user_response.body_string().expect("Response Body");
    let user1: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    
    // New user 2
    let new_user_response = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jules A. Doe",
            "email": "julz@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(new_user_response.status(), Status::Ok);
    
    // Login user 1
    let mut login_response = client.post("/api/login")
        .header(ContentType::JSON)
        .body(r##"{
            "email": "juvied@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(login_response.status(), Status::Ok);
    
    // Extract info on user logged in
    let response_body = login_response.body_string().expect("Response Body");
    let login_auth: Authenticated = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    
    // Test Login 1
    let response_cookies = login_response.cookies();
    assert_eq!(response_cookies[0].name(), "t");
    assert_eq!(login_auth.id, user1.id);

    // Info on User 2
    let mut info_response = client.get("/api/users/julz@m.com").dispatch();
    
    // Test it is denied altogether
    assert_eq!(info_response.status(), Status::InternalServerError);
    assert_eq!(info_response.body_string(), Some(format!("\"user julz@m.com not found\"")));


    // Cleanup User 1
    let res = client.delete("/api/users/")
        .header(ContentType::JSON)
        .body(r##"{
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    
    // Cleanup User 2 (login first)
    let login_response = client.post("/api/login")
        .header(ContentType::JSON)
        .body(r##"{
            "email": "julz@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(login_response.status(), Status::Ok);
    let res = client.delete("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
}
