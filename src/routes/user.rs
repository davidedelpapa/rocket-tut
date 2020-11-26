use std::collections::HashMap;
use chrono::{DateTime, Utc};
use rocket::*;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;
use rocket_contrib::uuid::Uuid;
use uuid::Uuid as Uuid2;
use rocket::response;
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use r2d2_redis::redis as redis;
use redis::Commands;
use anyhow::Result as AnyResult;
use anyhow::anyhow;
use crate::data::db::{User, InsertableUser, ResponseUser, UserPassword};
use crate::data::redis_connection::Conn;

const LOOKUP: &str = "email_lookup";

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
    pub fn internal_err() -> Self {
        ApiResponse {
            status: Status::InternalServerError,
            message: json!("Internal server error"),
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

impl User {
    fn from_redis(connection: &mut Conn, id: &String) -> AnyResult<Self> {
        let r_user: HashMap<String, String>  = connection.hgetall(id)?;
        
        let r_user_id = Uuid2::parse_str(&* id)?;
        
        let r_user_name: &String = r_user.get(&"name".to_string()).ok_or(anyhow!(""))?;
        let r_user_email: &String = r_user.get(&"email".to_string()).ok_or(anyhow!(""))?;
        let r_user_hashed_password: &String = r_user.get(&"hashed_password".to_string()).ok_or(anyhow!(""))?;
        let r_user_salt: &String = r_user.get(&"salt".to_string()).ok_or(anyhow!(""))?;
        let r_user_created: &String = r_user.get(&"created".to_string()).ok_or(anyhow!(""))?;
        let r_user_updated: &String = r_user.get(&"updated".to_string()).ok_or(anyhow!(""))?;

        let created: DateTime<Utc> = r_user_created.parse()?;
        let updated: DateTime<Utc> = r_user_updated.parse()?;

        Ok(User {
            id: r_user_id,
            name: r_user_name.to_owned(),
            email: r_user_email.to_owned(),
            hashed_password: r_user_hashed_password.to_owned(),
            salt: r_user_salt.to_owned(),
            created,
            updated,
        })
    }

    fn to_redis(self, connection: &mut Conn) -> AnyResult<()> {
        let id = self.id.to_string();
        let email = self.email.clone();
        let r_user = [
            ("name", self.name),
            ("email", self.email),
            ("hashed_password", self.hashed_password),
            ("salt", self.salt),
            ("created", self.created.to_string()),
            ("updated", self.updated.to_string())
        ];
        connection.hset_multiple(&id, &r_user)?;
        // Add email lookup index
        let _ = connection.zadd(LOOKUP, format!("{}:{}", email, id), 0)?;
        Ok(())
    }
}


#[get("/users")]
pub fn user_list_rt(mut connection: Conn) -> ApiResponse {
    let connection = &mut *connection;
    let connection_raw: &mut r2d2_redis::redis::Connection = &mut *connection;
    let users_keys: Result<i32, _> = redis::cmd("DBSIZE").query(connection_raw);
    match users_keys {
        Ok(mut user_size) => {
            if user_size >= 2 {user_size -=1 };
            ApiResponse::ok(json!([user_size]))
        },
        Err(_) => ApiResponse::internal_err(),
    }
}

#[post("/users", format = "json", data = "<user>")]
pub fn new_user_rt(mut connection: Conn, user: Json<InsertableUser>) -> ApiResponse {
    let ins_user = User::from_insertable((*user).clone());
    match ins_user.clone().to_redis(&mut connection){
        Ok(_) => ApiResponse::ok(json!(ResponseUser::from_user(&ins_user))),
        Err(_) => ApiResponse::internal_err(),
    }
}

#[get("/users/<id>")]
pub fn info_user_rt(mut connection: Conn, id: Uuid) -> ApiResponse {
    let id = id.to_string();
    match User::from_redis(&mut connection, &id){
        Ok(user) => ApiResponse::ok(json!(ResponseUser::from_user(&user))),
        Err(_) => ApiResponse::err(json!(format!("id {} not found", id))),
    }
}

#[put("/users/<id>", format = "json", data = "<user>")]
pub fn update_user_rt(mut connection: Conn, user: Json<InsertableUser>, id: Uuid) -> ApiResponse {
    let id = id.to_string();
    match User::from_redis(&mut connection, &id){
        Ok(user_from_redis) =>{
            let mut user_to_redis = user_from_redis.clone();
            if user_to_redis.match_password(&user.password) {
                let _res_lookup: Result<i32, _> = connection.zrem(LOOKUP, format!("{}:{}", user_from_redis.email, id));
                let insert_user = user_to_redis.update_user(&user.name, &user.email);
                match insert_user.clone().to_redis(&mut connection) {
                    Ok(_) => ApiResponse::ok(json!(ResponseUser::from_user(&insert_user))),
                    Err(_) => ApiResponse::internal_err(),
                }
            }
            else { ApiResponse::err(json!("user not authenticated")) }
        },
        Err(_) => ApiResponse::err(json!(format!("id {} not found", id)))
    }
}

#[delete("/users/<id>", format = "json", data = "<user>")]
pub fn delete_user_rt(mut connection: Conn, user: Json<UserPassword>, id: Uuid) -> ApiResponse {
    let id = id.to_string();
    match User::from_redis(&mut connection, &id){
        Ok(user_from_redis) =>{
            if user_from_redis.match_password(&user.password) {
                let res: Result<i32, _> = connection.del(&id);
                let _res_lookup: Result<i32, _> = connection.zrem(LOOKUP, format!("{}:{}", user_from_redis.email, id));
                match res {
                    Ok(_) => ApiResponse::ok(json!(ResponseUser::from_user(&user_from_redis))),
                    Err(_) => ApiResponse::internal_err(),
                }
            }
            else { ApiResponse::err(json!("user not authenticated")) }
        },
        Err(_) => ApiResponse::err(json!(format!("id {} not found", id)))
    }
}

#[patch("/users/<id>", format = "json", data = "<user>")]
pub fn patch_user_rt(mut connection: Conn, user: Json<UserPassword>, id: Uuid) -> ApiResponse {
    match &user.new_password {
        Some(passw) => {
            let id = id.to_string();
            match User::from_redis(&mut connection, &id){
                Ok(mut user_from_redis) =>{
                    if user_from_redis.clone().match_password(&user.password) {
                        let insert_user = user_from_redis.update_password(&passw);
                        let _res_lookup: Result<i32, _> = connection.zrem(LOOKUP, format!("{}:{}", user_from_redis.email, id));
                        match insert_user.clone().to_redis(&mut connection) {
                            Ok(_) => ApiResponse::ok(json!("Password updated")),
                            Err(_) => ApiResponse::internal_err(),
                        }
                    }
                    else { ApiResponse::err(json!("user not authenticated")) }
                },
                Err(_) => ApiResponse::err(json!(format!("id {} not found", id))),
            }
        },
        None => ApiResponse::err(json!("Password not provided"))
    }
}

#[get("/users/<email>", rank = 2)]
pub fn id_user_rt(mut connection: Conn, email: String) -> ApiResponse {
    let get_item: Result<Vec<String>, _> = connection.zrangebylex(LOOKUP, format!("[{}:", &email), "+");
    match get_item {
        Ok(lookup_vector) => {
            if lookup_vector.is_empty(){
                return ApiResponse::err(json!(format!("user {} not found",  &email)));
            }
            let split = lookup_vector[0].split(":").collect::<Vec<&str>>();
            let id = split[1].to_string();
            match User::from_redis(&mut connection, &id){
                Ok(user) => ApiResponse::ok(json!(ResponseUser::from_user(&user))),
                Err(_) => ApiResponse::err(json!(format!("user {} not found",  &email))),
            }
        },
        Err(_) => ApiResponse::internal_err()
    }
}
