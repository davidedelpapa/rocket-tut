use rocket::local::Client;
use rocket_tut::rocket_builder;
use rocket::http::{ContentType, Status};
use rocket_tut::data::db::ResponseUser;
use rocket_tut::routes::auth::Authenticated;
use serde_json;


#[test]
fn ping_test() {
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");
    
    // Test /ping
    let mut response = client.get("/ping").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("PONG!".into()));
}

#[test]
fn user_list_rt_test(){
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");

    // New user
    let new_user_response = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jeremy Doe",
            "email": "jerrydoe88@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(new_user_response.status(), Status::Ok);
    
    // Login
    let login_response = client.post("/api/login")
        .header(ContentType::JSON)
        .body(r##"{
            "email": "jerrydoe88@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(login_response.status(), Status::Ok);
        
    // Test user_list_rt
    let mut response = client.get("/api/users").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let mut response_body = response.body_string().unwrap();
    response_body.retain(|c| !c.is_numeric());
    assert_eq!(response_body, "[]");
    
    // Cleanup
    let res = client.delete("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);    
}

#[test]
fn new_user_rt_test(){
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");

    // New user (test new_user_rt)
    let mut response = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "John Doe",
            "email": "j.doe@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let response_body = response.body_string().expect("Response Body");
    let user: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "j.doe@m.com");

    // Login (in order to clean up)
    let login_response = client.post("/api/login")
        .header(ContentType::JSON)
        .body(r##"{
            "email": "j.doe@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(login_response.status(), Status::Ok);

    // Cleanup
    let res = client.delete("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
}

#[test]
fn info_user_rt_test(){
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");

    // New user
    let mut response_new_user = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jane Doe",
            "email": "jane.doe@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(response_new_user.status(), Status::Ok);

    let response_body = response_new_user.body_string().expect("Response Body");
    let user_new: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    let id = user_new.id;

    // Login
    let mut login_response = client.post("/api/login")
        .header(ContentType::JSON)
        .body(r##"{
            "email": "jane.doe@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(login_response.status(), Status::Ok);
    assert_eq!(login_response.content_type(), Some(ContentType::JSON));
    let response_body = login_response.body_string().expect("Response Body");
    let login_auth: Authenticated = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    assert_eq!(login_auth.id, id);

    // Test info_user_rt()
    let mut response = client.get(format!("/api/users/{}", id)).dispatch();
    let response_body = response.body_string().expect("Response Body");
    let user: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(user.name, "Jane Doe");
    assert_eq!(user.email, "jane.doe@m.com");
    assert_eq!(user.id, id);

    // Cleanup
    let res = client.delete("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
}

#[test]
fn update_user_rt_test(){
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");

    // New user
    let mut response_new_user = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jak Doe",
            "email": "jack.doe@m.com",
            "password": "quertyuiop"
        }"##)
        .dispatch();
    let response_body = response_new_user.body_string().expect("Response Body");
    let user_new: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    let id = user_new.id;

    // Login
    let login_response = client.post("/api/login")
        .header(ContentType::JSON)
        .body(r##"{
            "email": "jack.doe@m.com",
            "password": "quertyuiop"
        }"##)
        .dispatch();
    assert_eq!(login_response.status(), Status::Ok);
    
    // Test update_user_rt
    let mut response = client.put("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jack Doe",
            "email": "jkd@m.com",
            "password": "quertyuiop"
        }"##)
        .dispatch();
    let response_body = response.body_string().expect("Response Body");
    let user: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(user.name, "Jack Doe");
    assert_eq!(user.email, "jkd@m.com");
    assert_eq!(user.id, id);
    
    // Cleanup
    let res = client.delete("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "password": "quertyuiop"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
}

#[test]
fn delete_user_rt_test(){
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");

    // New user
    let mut response_new_user = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jerome Doe",
            "email": "j85@m.com",
            "password": "asdfghjkl"
        }"##)
        .dispatch();
    let response_body = response_new_user.body_string().expect("Response Body");
    let user_new: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    let id = user_new.id;

    // Login
    let login_response = client.post("/api/login")
        .header(ContentType::JSON)
        .body(r##"{
            "email": "j85@m.com",
            "password": "asdfghjkl"
        }"##)
        .dispatch();
    assert_eq!(login_response.status(), Status::Ok);

    // Test delete_user_rt
    let mut response = client.delete("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "password": "asdfghjkl"
        }"##)
        .dispatch();
    let response_body = response.body_string().expect("Response Body");
    let user: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(user.name, "Jerome Doe");
    assert_eq!(user.email, "j85@m.com");
    assert_eq!(user.id, id);
}

#[test]
fn patch_user_rt_test(){
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");

    // New user
    let response_new_user = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jonathan Doe",
            "email": "jondon@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(response_new_user.status(), Status::Ok);

    // Login
    let login_response = client.post("/api/login")
        .header(ContentType::JSON)
        .body(r##"{
            "email": "jondon@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(login_response.status(), Status::Ok);
    
    // Test patch_user_rt
    let mut response = client.patch("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "password": "123456",
            "new_password": "quertyuiop"
        }"##)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("\"Password updated\"".into()));

    // Cleanup
    let res = client.delete("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "password": "quertyuiop"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
}

#[test]
fn id_user_rt_test(){
    let client = Client::new(rocket_builder()).expect("Valid Rocket instance");

    // New user
    let mut response_new_user = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Janet Doe",
            "email": "janet.doe@m.com",
            "password": "zxcvbnm"
        }"##)
        .dispatch();
    let response_body = response_new_user.body_string().expect("Response Body");
    let user_new: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    let id = user_new.id;

    // Login
    let login_response = client.post("/api/login")
        .header(ContentType::JSON)
        .body(r##"{
            "email": "janet.doe@m.com",
            "password": "zxcvbnm"
        }"##)
        .dispatch();
    assert_eq!(login_response.status(), Status::Ok);
    
    // Test id_user_rt
    let mut response = client.get(format!("/api/users/{}", "janet.doe@m.com")).dispatch();
    let response_body = response.body_string().expect("Response Body");
    let user: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(user.name, "Janet Doe");
    assert_eq!(user.id, id);

    // Cleanup
    let res = client.delete("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "password": "zxcvbnm"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);

}
