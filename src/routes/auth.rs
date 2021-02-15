use rocket::*;
use rocket::http::{Cookies, Cookie};
use rocket_contrib::json::Json;
use rocket_contrib::json;
use serde::{Deserialize, Serialize};

use r2d2_mongodb::mongodb as bson;
use r2d2_mongodb::mongodb as mongodb;

use bson::{bson, doc, Bson};
use mongodb::db::ThreadedDatabase;

use crate::data::security;
use crate::data::mongo_connection::Conn;
use crate::data::db::User;
use crate::routes::responses::ApiResponse;

// TODO in.env
const COLLECTION: &str = "users";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
struct Authenticated {
    id: String,
}

#[post("/login", format = "json", data = "<login>")]
pub fn login_user(connection: Conn, login: Json<LoginUser>, mut cookies: Cookies) -> ApiResponse {
    let user_coll = &connection.collection(COLLECTION);
    match user_coll.find_one(Some(doc! { "email": login.email.clone() }), None) {
        Ok(find_one) => {
            match find_one {
                Some(found_user) => {
                    let loaded_user_doc: Result<User, _> = bson::from_bson(Bson::Document(found_user));
                    match loaded_user_doc {
                        Ok(got_user) => {
                            if got_user.match_password(&login.password) {
                                let id = got_user.id.to_string();
                                let cookie = security::sign_token(id.clone());
                                match cookie {
                                    Ok(c) => {
                                        cookies.add(Cookie::new("t", c));
                                        ApiResponse::ok(json!(Authenticated {
                                            id,
                                        }))
                                    },
                                    Err(_) => ApiResponse::err(json!("Could not set cookies"))
                                }
                            }
                            else { ApiResponse::err(json!("Invalid password")) }
                        }
                        Err(_) => ApiResponse::internal_err(),
                    }                    
                },
                None => ApiResponse::err(json!(format!("user {} not found",  login.email))),
            }
        },
        Err(_) => ApiResponse::internal_err(),
    }
}
