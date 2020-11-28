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
            "name": "Johnny Doe",
            "email": "johnny.doe@m.com",
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
            "name": "Jay Doe",
            "email": "jay.doe@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    let response_body = response_new_user.body_string().expect("Response Body");
    let user_new: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    let mut id = user_new.id.clone();
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

    // Cleanup
    let res = client.delete(format!("/api/users/{}", user_new.id))
        .header(ContentType::JSON)
        .body(r##"{
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
}

#[test]
fn update_user_rt_fail(){
    let client = common::setup();
    // New insertion must be correct
    let mut response_new_user = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jack S. Doe",
            "email": "jack.s.doe@m.com",
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
            "name": "Jack S. Doe",
            "email": "jack.s.doe@m.com",
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
            "name": "Jack S. Doe",
            "email": "jack.s.doe@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_ne!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.body_string(), Some("\"user not authenticated\"".to_string()));

    // Cleanup
    let res = client.delete(format!("/api/users/{}", id))
        .header(ContentType::JSON)
        .body(r##"{
            "password": "quertyuiop"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
}

#[test]
fn delete_user_rt_fail(){
    let client = common::setup();
    let mut response_new_user = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jerome M. Doe",
            "email": "jm85@m.com",
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
    
    // Cleanup
    let res = client.delete(format!("/api/users/{}", id))
        .header(ContentType::JSON)
        .body(r##"{
            "password": "asdfghjkl"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
}

#[test]
fn patch_user_rt_fail(){
    let client = common::setup();
    let mut response_new_user = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jonathan M. Doe",
            "email": "jondonmagic@m.com",
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
    
    // Cleanup
    let res = client.delete(format!("/api/users/{}", id))
        .header(ContentType::JSON)
        .body(r##"{
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
}

#[test]
fn id_user_rt_fail(){
    let client = common::setup();
    let mut response_new_user = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Janet Eveline Doe",
            "email": "janetev.doe@m.com",
            "password": "zxcvbnm"
        }"##)
        .dispatch();
    // We have to make sure this does not fail because of wrong new user insertion
    assert_eq!(response_new_user.status(), Status::Ok);
    assert_eq!(response_new_user.content_type(), Some(ContentType::JSON));
    let response_body = response_new_user.body_string().expect("Response Body");
    let user: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    assert_eq!(user.name, "Janet Eveline Doe");
    assert_eq!(user.email, "janetev.doe@m.com");

    // First test: wrong email == no user found
    let mut response = client.get(format!("/api/users/{}", "janetta@l.com")).dispatch();

    let answer = response.body_string();
    println!("{:?}", &answer);
    assert_ne!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(answer, Some(format!("\"user janetta@l.com not found\"")));

    // Cleanup
    let res = client.delete(format!("/api/users/{}", user.id))
        .header(ContentType::JSON)
        .body(r##"{
            "password": "zxcvbnm"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
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

#[test]
fn unique_emails_insertion_fail(){
    let client = common::setup();

    // First user with its email
    let mut response_new_user = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jared Doe",
            "email": "jthebest@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    // We have to make sure this does not fail because of wrong new user insertion
    assert_eq!(response_new_user.status(), Status::Ok);
    assert_eq!(response_new_user.content_type(), Some(ContentType::JSON));
    let response_body = response_new_user.body_string().expect("Response Body");
    let user: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    
    // Second user with the same email
    let mut response_second_user = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Joy Doe",
            "email": "jthebest@m.com",
            "password": "qwertyuiop"
        }"##)
        .dispatch();
    
    assert_ne!(response_second_user.status(), Status::Ok);
    assert_eq!(response_second_user.content_type(), Some(ContentType::JSON));
    assert_eq!(response_second_user.body_string(), Some("\"email already in use\"".to_string()));

    // Cleanup
    let res = client.delete(format!("/api/users/{}", user.id))
        .header(ContentType::JSON)
        .body(r##"{
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
}

#[test]
fn unique_emails_update_fail(){
    let client = common::setup();

    // First user with its email
    let mut response_first_user = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Joe Doe",
            "email": "jeffreyd@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    // We have to make sure this does not fail because of wrong user insertion
    assert_eq!(response_first_user.status(), Status::Ok);
    assert_eq!(response_first_user.content_type(), Some(ContentType::JSON));
    let response_body = response_first_user.body_string().expect("Response Body");
    let first_user: ResponseUser = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    let first_id = first_user.id;

    // Second user, with its other email
    let mut response_second_user = client.post("/api/users")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Jolanda Doe",
            "email": "jo_me@m.com",
            "password": "qwertyuiop"
        }"##)
        .dispatch();
    // We have to make sure this does not fail because of wrong  user insertion
    assert_eq!(response_second_user.status(), Status::Ok);
    assert_eq!(response_second_user.content_type(), Some(ContentType::JSON));
    let response2_body = response_second_user.body_string().expect("Response Body");
    let second_user: ResponseUser = serde_json::from_str(&response2_body.as_str()).expect("Valid User Response");
    let second_id = second_user.id;

    // We change the first user to have the same email as the second one
    let mut response = client.put(format!("/api/users/{}", first_id))
        .header(ContentType::JSON)
        .body(r##"{
            "name": "Joe K. Doe",
            "email": "jo_me@m.com",
            "password": "123456"
        }"##)
        .dispatch();
    assert_ne!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.body_string(), Some("\"email already in use\"".to_string()));

    // Cleanup (double trouble)
    let res1 = client.delete(format!("/api/users/{}", first_id))
        .header(ContentType::JSON)
        .body(r##"{
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(res1.status(), Status::Ok);
    let res2 = client.delete(format!("/api/users/{}", second_id))
        .header(ContentType::JSON)
        .body(r##"{
            "password": "qwertyuiop"
        }"##)
        .dispatch();
    assert_eq!(res2.status(), Status::Ok);
}