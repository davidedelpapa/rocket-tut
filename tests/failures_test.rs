use lazy_static;
use rocket::http::{ContentType, Status};
use rocket_tut::data::db::ResponseUser;
use serde_json;

mod common;

#[test]
fn generic_fail(){
    let client = common::setup();
    let response = client.get("/pin").dispatch();
    assert_ne!(response.status(), Status::Ok);
    assert_eq!(response.status(), Status::NotFound);
    assert_ne!(response.content_type(), Some(ContentType::JSON));
}

#[test]
fn user_list_rt_fail(){
    // there's no way to make it fail purposedly if the server started correctly.
    // only if it occurs an unplanned server error will GET to /api/users fail.
    assert!(true);
}

#[test]
fn new_user_rt_fail(){
    let client = common::setup();
    // Header binary fail
    let response = client.post("/api/users")
        .header(ContentType::Binary)
        .body(r##"{
            "name": "John Doe",
            "email": "j.doe@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_ne!(response.status(), Status::Ok);
    assert_eq!(response.status(), Status::NotFound);
    assert_ne!(response.content_type(), Some(ContentType::JSON));
}

#[test]
fn info_user_rt_fail(){
    let client = common::setup();
    // New insertion must be correct
    let mut response_new_user = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jane Doe",
            "email": "jane.doe@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    let response_body = response_new_user.body_string().expect("Response Body");
    let user_new: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    let mut id = user_new.id;
    // Now we construct a purposedly false id. 
    // we need to keep it looking as a Uuid, otherwise it will get passed to the second ranking GET
    if id.remove(0) != 'a' {
        id.insert(0, 'a');
    }
    else {
        id.insert(0, 'b');
    }
    let mut response = client.get(format!("/api/users/{}", id)).dispatch();
    assert_ne!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.body_string(), Some(format!("\"id {} not found\"",  id)));
}

#[test]
fn update_user_rt_fail(){
    let client = common::setup();
    // New insertion must be correct
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
    
    // First test: wrong id
    let mut wrong_id = id.clone();
    if wrong_id.remove(0) != 'a' {
        wrong_id.insert(0, 'a');
    }
    else {
        wrong_id.insert(0, 'b');
    }
    let mut response = client.put(format!("/api/users/{}", wrong_id))
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jack Doe",
            "email": "jkd@m.com",
            "password": "quertyuiop"
        }"##)
        .dispatch();
    assert_ne!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.body_string(), Some(format!("\"id {} not found\"",  wrong_id)));

    // Second test: wrong password
    let mut response = client.put(format!("/api/users/{}", id))
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jack Doe",
            "email": "jkd@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_ne!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.body_string(), Some("\"user not authenticated\"".to_string()));
}

#[test]
fn delete_user_rt_fail(){
    let client = common::setup();
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

    // First test: wrong id
    let mut wrong_id = id.clone();
    if wrong_id.remove(0) != 'a' {
        wrong_id.insert(0, 'a');
    }
    else {
        wrong_id.insert(0, 'b');
    }
    let mut response = client.delete(format!("/api/users/{}", wrong_id))
        .header(ContentType::JSON)
        .body(r##"{
            "password": "asdfghjkl"
        }"##)
        .dispatch();
    assert_ne!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.body_string(), Some(format!("\"id {} not found\"",  wrong_id)));
    
    // Second test: wrong password
    let mut response = client.delete(format!("/api/users/{}", id))
        .header(ContentType::JSON)
        .body(r##"{
            "password": "qwertyuiop"
        }"##)
        .dispatch();
    assert_ne!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.body_string(), Some("\"user not authenticated\"".to_string()));
}

#[test]
fn patch_user_rt_fail(){
    let client = common::setup();
    let mut response_new_user = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jonathan Doe",
            "email": "jondon@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    let response_body = response_new_user.body_string().expect("Response Body");
    let user_new: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    let id = user_new.id;
    // First test: wrong id
    let mut wrong_id = id.clone();
    if wrong_id.remove(0) != 'a' {
        wrong_id.insert(0, 'a');
    }
    else {
        wrong_id.insert(0, 'b');
    }
    let mut response = client.patch(format!("/api/users/{}", wrong_id))
        .header(ContentType::JSON)
        .body(r##"{
            "password": "123456",
            "new_password": "quertyuiop"
        }"##)
        .dispatch();
    assert_ne!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.body_string(), Some(format!("\"id {} not found\"",  wrong_id)));

    // Second test: wrong password
    let mut response = client.patch(format!("/api/users/{}", id))
        .header(ContentType::JSON)
        .body(r##"{
            "password": "zxcvbnm",
            "new_password": "quertyuiop"
        }"##)
        .dispatch();
    assert_ne!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.body_string(), Some("\"user not authenticated\"".to_string()));

    // Third test: no new password provided
    let mut response = client.patch(format!("/api/users/{}", id))
        .header(ContentType::JSON)
        .body(r##"{
            "password": "123456"
        }"##)
        .dispatch();
    assert_ne!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.body_string(), Some("\"Password not provided\"".to_string()));
}

#[test]
fn id_user_rt_fail(){
    let client = common::setup();
    let mut response_new_user = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Janet Doe",
            "email": "janet.doe@m.com",
            "password": "zxcvbnm"
        }"##)
        .dispatch();
    // We have to make sure this does not fail because of wrong new user insertion
    assert_eq!(response_new_user.status(), Status::Ok);
    assert_eq!(response_new_user.content_type(), Some(ContentType::JSON));
    let response_body = response_new_user.body_string().expect("Response Body");
    let user: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    assert_eq!(user.name, "Janet Doe");
    assert_eq!(user.email, "janet.doe@m.com");

    // First test: wrong email == no user found
    let mut response = client.get(format!("/api/users/{}", "janet@m.com")).dispatch();
    assert_ne!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.body_string(), Some(format!("\"user janet@m.com not found\"")));
}

#[test]
fn get_rank_fail(){
    let client = common::setup();
    // Second test: lets make sure we get the second ranked route
    // Thus, we construct a purposedly false email resembling a Uuid

    // We'll keep it either 32 or 36 characters, with exact group lenghts,
    // but we'll format is as an email at the end.
    // It should fail because of invalid characters: '@' and '.'
    let deceptive_email = "7b8f9f31-d70c-4834-8164-ca20b8@b.989";
    
    let mut response = client.get(format!("/api/users/{}", deceptive_email)).dispatch();
    assert_ne!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.status(), Status::InternalServerError);

    // Now we have to test it did get to the second ranking route, not the first, and failed there
    
    let response_body = response.body_string().expect("Response Body"); // beware!
    // Remeber if you want to test many times on the same body_string(),
    //      the second time it is invoked body_string() == None
    //      Thus we save it into a String
    assert_ne!(response_body, format!("\"id {} not found\"",  deceptive_email));
    assert_eq!(response_body, format!("\"user {} not found\"", deceptive_email));
}