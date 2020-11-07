use rocket::*;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;
use rocket_contrib::uuid::Uuid;
use rocket::State;
use rocket::response;
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use crate::Users;
use crate::data::db::{User, InsertableUser, ResponseUser, UserPassword};

#[derive(Debug)]
pub struct ApiResponse {
    status: Status,
    message: JsonValue,
}
impl ApiResponse {
    pub fn ok(message: JsonValue) -> Self {
        ApiResponse {
            status: Status::Ok,
            message: message,
        }
    }
    pub fn err(message: JsonValue) -> Self {
        ApiResponse {
            status: Status::InternalServerError,
            message: message,
        }
    }
}
impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.message.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[get("/users")]
pub fn user_list_rt(userdb: State<Users>) -> ApiResponse {
    let v = userdb.db.lock().unwrap();
    let users = &*v;
    ApiResponse::ok(json!([users.len()]))
}

#[post("/users", format = "json", data = "<user>")]
pub fn new_user_rt(userdb: State<Users>, user: Json<InsertableUser>) -> ApiResponse {
    let mut v = userdb.db.lock().unwrap();
    let users = &mut *v;
    users.push(User::from_insertable((*user).clone()));
    ApiResponse::ok(json!(ResponseUser::from_user(&users.last().unwrap())))
}

#[get("/users/<id>")]
pub fn info_user_rt(userdb: State<Users>, id: Uuid) -> ApiResponse {
    let mut v = userdb.db.lock().unwrap();
    let users = &mut *v;
    let pos = users.iter().position(|x| x.id.to_string() == id.to_string());
    match pos {
        Some(p) => ApiResponse::ok(json!(ResponseUser::from_user(&v[p]))),
        None => ApiResponse::err(json!(format!("id {} not found",  id)))
    }
}

#[put("/users/<id>", format = "json", data = "<user>")]
pub fn update_user_rt(userdb: State<Users>, user: Json<InsertableUser>, id: Uuid) -> ApiResponse {
    let mut v = userdb.db.lock().unwrap();
    let users = &mut *v;
    let pos = users.iter().position(|x| x.id.to_string() == id.to_string());
    match pos {
        Some(p) => {
            if v[p].match_password(&user.password) {
                v[p].update_user(&user.name, &user.email);
                ApiResponse::ok(json!(ResponseUser::from_user(&v[p])))
            }
            else { ApiResponse::err(json!("user not authenticated")) }
        },
        None => ApiResponse::err(json!(format!("id {} not found",  id)))
    }
}

#[delete("/users/<id>", format = "json", data = "<user>")]
pub fn delete_user_rt(userdb: State<Users>, user: Json<UserPassword>, id: Uuid) -> ApiResponse {
    let mut v = userdb.db.lock().unwrap();
    let users = &mut *v;
    let pos = users.iter().position(|x| x.id.to_string() == id.to_string());
    match pos {
        Some(p) => {
            if v[p].match_password(&user.password) {
                let u = v[p].clone();
                v.remove(p);
                ApiResponse::ok(json!(ResponseUser::from_user(&u)))
            }
            else { ApiResponse::err(json!("user not authenticated")) }
        },
        None => ApiResponse::err(json!(format!("id {} not found",  id)))
    }
}

#[patch("/users/<id>", format = "json", data = "<user>")]
pub fn patch_user_rt(userdb: State<Users>, user: Json<UserPassword>, id: Uuid) -> ApiResponse {
    let mut v = userdb.db.lock().unwrap();
    let users = &mut *v;
    let pos = users.iter().position(|x| x.id.to_string() == id.to_string());
    match pos {
        Some(p) => {
            if v[p].match_password(&user.password) {
                match &user.new_password {
                    Some(passw) => {
                        v[p].update_password(&passw);
                        ApiResponse::ok(json!("Password updated"))
                    },
                    None => ApiResponse::err(json!("Password not provided"))
                }
            }
            else { ApiResponse::err(json!("user not authenticated")) }
        },
        None => ApiResponse::err(json!(format!("id {} not found",  id)))
    }
}

#[get("/users/<email>", rank = 2)]
pub fn id_user_rt(userdb: State<Users>, email: String) -> ApiResponse {
    let mut v = userdb.db.lock().unwrap();
    let users = &mut *v;
    let pos = users.iter().position(|x| x.email == email);
    match pos {
        Some(p) => ApiResponse::ok(json!(ResponseUser::from_user(&v[p]))),
        None => ApiResponse::err(json!(format!("user {} not found",  email)))
    }
}
