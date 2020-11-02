use rocket::http::{ContentType, Status};

mod common;

#[test]
fn ping_test() {
    let client = common::setup();
    let mut response = client.get("/ping").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("PONG!".into()));
}

#[test]
fn user_list_rt_test(){
    let client = common::setup();
    let mut response = client.get("/api/users").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"List of users\"}".into()));
}

#[test]
fn new_user_rt_test(){
    let client = common::setup();
    let mut response = client.post("/api/users").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"Creation of new user\"}".into()));
}

#[test]
fn info_user_rt_test(){
    let client = common::setup();
    let mut response = client.get("/api/users/1").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"Info for user 1\"}".into()));
}

#[test]
fn update_user_rt_test(){
    let client = common::setup();
    let mut response = client.put("/api/users/1").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"Update info for user 1\"}".into()));
}

#[test]
fn delete_user_rt_test(){
    let client = common::setup();
    let mut response = client.delete("/api/users/1").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"Delete user 1\"}".into()));
}
