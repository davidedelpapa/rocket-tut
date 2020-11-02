use rocket::*;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Response {
    status: String,
    message: String,
}
impl Response {
    pub fn ok(msg: &str) -> Self {
        Response {
            status: "Success".to_string(),
            message: msg.to_string(),
        }
    }
    pub fn err(msg: &str) -> Self {
        Response {
            status: "Error".to_string(),
            message: msg.to_string(),
        }
    }
}

#[get("/users")]
pub fn user_list_rt() -> Json<Response> {
    Json(Response::ok("List of users"))
}

#[post("/users")]
pub fn new_user_rt() -> Json<Response> {
    Json(Response::ok("Creation of new user"))
}

#[get("/users/<id>")]
pub fn info_user_rt(id: String) -> Json<Response> {
    Json(Response::ok(&* format!("Info for user {}", id)))
}

#[put("/users/<id>")]
pub fn update_user_rt(id: String) -> Json<Response> {
    Json(Response::ok(&* format!("Update info for user {}", id)))
}

#[delete("/users/<id>")]
pub fn delete_user_rt(id: String) -> Json<Response> {
    Json(Response::ok(&* format!("Delete user {}", id)))
}