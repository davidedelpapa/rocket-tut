use rocket::*;
use rocket_contrib::json::Json;
use rocket_contrib::json;
use rocket_contrib::uuid::Uuid;

use r2d2_mongodb::mongodb as bson;
use r2d2_mongodb::mongodb as mongodb;

use bson::{bson, doc, Bson};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::options::{ReturnDocument, FindOneAndUpdateOptions};

use crate::data::db::{User, InsertableUser, ResponseUser, UserPassword};
use crate::data::mongo_connection::Conn;
use crate::routes::responses::ApiResponse;
use crate::data::security::JwtGuard;

const COLLECTION: &str = "users";


#[get("/users")]
pub fn user_list_rt(connection: Conn, _guard : JwtGuard) -> ApiResponse {
    let user_coll = &connection.collection(COLLECTION);
    match  user_coll.count(None, None) {
        Ok(res) => ApiResponse::ok(json!([res])),
        Err(_) => ApiResponse::internal_err(),
    }   
}

#[post("/users", format = "json", data = "<user>")]
pub fn new_user_rt(connection: Conn, user: Json<InsertableUser>) -> ApiResponse {
    let user_coll = &connection.collection(COLLECTION);
    match bson::to_bson(&User::from_insertable((*user).clone())) {
        Ok(serialized) => {
            match serialized.as_document() {
                Some(document) => {
                    match user_coll.insert_one(document.to_owned(), None) {
                        Ok(inserted) => {
                            match inserted.inserted_id {
                                Some(id) => {
                                    match user_coll.find_one(Some(doc! { "_id":  id }), None) {
                                        Ok(find_one) => {
                                            match find_one {
                                                Some(found_user) => {
                                                    let loaded_user_doc: Result<User, _> = bson::from_bson(Bson::Document(found_user));
                                                    match loaded_user_doc {
                                                        Ok(loaded_user) => ApiResponse::ok(json!(ResponseUser::from_user(&loaded_user))),
                                                        Err(_) => ApiResponse::internal_err(),
                                                    }
                                                },
                                                None => ApiResponse::internal_err(),
                                            }
                                        },
                                        Err(_) => ApiResponse::internal_err(),
                                    }
                                },
                                None => match inserted.write_exception {
                                    Some(wite_error) =>{
                                        match wite_error.write_error {
                                            Some(err) =>{
                                                match err.code {
                                                    11000 => ApiResponse::err(json!("email already in use")),
                                                    _ => ApiResponse::internal_err(),
                                                }
                                            },
                                            None => ApiResponse::internal_err(),
                                        }
                                    },
                                    None => ApiResponse::internal_err(),
                                }
                            }
                        },                    
                        Err(_) => ApiResponse::internal_err(),
                    }
                },
                None => ApiResponse::internal_err(),
            }
        },
        Err(_) => ApiResponse::internal_err(),
    }
}

#[get("/users/<id>")]
pub fn info_user_rt(connection: Conn, id: Uuid, _guard : JwtGuard) -> ApiResponse {
    let user_coll = &connection.collection(COLLECTION);
    let id =  id.to_string();
    match user_coll.find_one(Some(doc! { "_id": id.clone() }), None) {
        Ok(find_one) => {
            match find_one {
                Some(found_user) => {
                    let found_user_doc: Result<User, _> = bson::from_bson(Bson::Document(found_user));
                    match found_user_doc {
                        Ok(found_user) => ApiResponse::ok(json!(ResponseUser::from_user(&found_user))),
                        Err(_) => ApiResponse::internal_err(),
                    }
                }
                None => ApiResponse::err(json!(format!("id {} not found",  id)))
            }
        },
        Err(_) => ApiResponse::internal_err(),
    }
}

#[put("/users/<id>", format = "json", data = "<user>")]
pub fn update_user_rt(connection: Conn, user: Json<InsertableUser>, id: Uuid, _guard : JwtGuard) -> ApiResponse {
    let user_coll = &connection.collection(COLLECTION);
    let id =  id.to_string();
    match user_coll.find_one(Some(doc! { "_id": id.clone() }), None) {
        Ok(find_one) => {
            match find_one {
                Some(found_user) => {
                    let found_user_doc: Result<User, _> = bson::from_bson(Bson::Document(found_user));
                    match found_user_doc {
                        Ok(mut found_user) => {
                            if found_user.match_password(&user.password) {
                                // Check the email does not yet exist
                                match user_coll.find_one(Some(doc! { "email": &user.email }), None) {
                                    Ok(mail_query_result) => {
                                        match mail_query_result {
                                            Some(_) => { return ApiResponse::err(json!("email already in use")); },
                                            None => ()
                                        }
                                    },
                                    Err(_) => { return ApiResponse::internal_err(); }
                                }
                                let insertable = found_user.update_user(&user.name, &user.email);
                                match bson::to_bson(&insertable) {
                                    Ok(serialized) => {
                                        let document = serialized.as_document().unwrap();
                                        let mut opt = FindOneAndUpdateOptions::new();
                                        opt.return_document = Some(ReturnDocument::After);
                                        match user_coll.find_one_and_replace(
                                            doc! { "_id": id.clone() },
                                            document.to_owned(),
                                            Some(opt)
                                        ) {
                                            Ok(updated_one) => {
                                                match updated_one {
                                                    Some(updated_user) => {
                                                        let updated_user_doc: Result<User, _> = bson::from_bson(Bson::Document(updated_user));
                                                        match updated_user_doc {
                                                            Ok(updated) => ApiResponse::ok(json!(ResponseUser::from_user(&updated))),
                                                            Err(_) => ApiResponse::internal_err(),
                                                        }                                                        
                                                    },
                                                    None => ApiResponse::err(json!(format!("id {} not found",  id))),
                                                }
                                            },                    
                                            Err(_) => ApiResponse::internal_err(),
                                        }
                                    },
                                    Err(_) => ApiResponse::internal_err(),
                                }
                            }
                            else { ApiResponse::err(json!("user not authenticated")) }
                        },
                        Err(_) => ApiResponse::internal_err(),
                    }
                },
                None => ApiResponse::err(json!(format!("id {} not found",  id))),
            }            
        },
        Err(_) => ApiResponse::internal_err(),
    }
}

#[delete("/users/<id>", format = "json", data = "<user>")]
pub fn delete_user_rt(connection: Conn, user: Json<UserPassword>, id: Uuid, _guard : JwtGuard) -> ApiResponse {
    let user_coll = &connection.collection(COLLECTION);
    let id =  id.to_string();
    match user_coll.find_one(Some(doc! { "_id": id.clone() }), None) {
        Ok(find_one) => {
            match find_one {
                Some(found_user) => {
                    let found_user_doc: Result<User, _> = bson::from_bson(Bson::Document(found_user));
                    match found_user_doc{
                        Ok(found_user) => {
                            if found_user.match_password(&user.password) {
                                match user_coll.find_one_and_delete(doc! { "_id": id.clone() }, None) {
                                    Ok(deleted_result) => {
                                        match deleted_result {
                                            Some(deleted_user) => {
                                                let deleted_doc: Result<User, _> = bson::from_bson(Bson::Document(deleted_user));
                                                match deleted_doc {
                                                    Ok(deleted) => ApiResponse::ok(json!(ResponseUser::from_user(&deleted))),
                                                    Err(_) => ApiResponse::internal_err(),
                                                }                                
                                            },
                                            None => ApiResponse::err(json!(format!("id {} not found",  id)))
                                        }
                                    },
                                    Err(_) => ApiResponse::internal_err(),
                                }
                            }
                            else { ApiResponse::err(json!("user not authenticated")) }
                        },
                        Err(_) => ApiResponse::internal_err(),
                    }
                },
                None => ApiResponse::err(json!(format!("id {} not found",  id)))
            }
        },
        Err(_) => ApiResponse::internal_err(),
    }    
}

#[patch("/users/<id>", format = "json", data = "<user>")]
pub fn patch_user_rt(connection: Conn, user: Json<UserPassword>, id: Uuid, _guard : JwtGuard) -> ApiResponse {
    let user_coll = &connection.collection(COLLECTION);
    let id =  id.to_string();
    match &user.new_password {
        Some(passw) => {
            match user_coll.find_one(Some(doc! { "_id": id.clone() }), None) {
                Ok(find_one) => {
                    match find_one {
                        Some(found_user) => {
                            let found_user_doc: Result<User, _> = bson::from_bson(Bson::Document(found_user));
                            match found_user_doc {
                                Ok(mut found_user) => {
                                    if found_user.match_password(&user.password) {
                                        let insertable = found_user.update_password(&passw);
                                        match bson::to_bson(&insertable) {
                                            Ok(serialized) => {
                                                let document = serialized.as_document().unwrap();
                                                match user_coll.find_one_and_replace(
                                                    doc! { "_id": id.clone() },
                                                    document.to_owned(),
                                                    None
                                                ) {
                                                    Ok(_) => ApiResponse::ok(json!("Password updated")),                   
                                                    Err(_) => ApiResponse::err(json!("Failed to update password")),
                                                }
                                            },
                                            Err(_) => ApiResponse::internal_err(),
                                        }
                                    }
                                    else { ApiResponse::err(json!("user not authenticated")) }
                                },
                                Err(_) => ApiResponse::internal_err(),
                            }
                        },
                        None => ApiResponse::err(json!(format!("id {} not found",  id)))
                    }
                },
                Err(_) => ApiResponse::internal_err(),
            }
        },
        None => ApiResponse::err(json!("Password not provided"))
    }
}

#[get("/users/<email>", rank = 2)]
pub fn id_user_rt(connection: Conn, email: String, _guard : JwtGuard) -> ApiResponse {
    let user_coll = &connection.collection(COLLECTION);
    match user_coll.find_one(Some(doc! { "email": email.clone() }), None) {
        Ok(find_one) => {
            match find_one {
                Some(found_user) => {
                    let loaded_user_doc: Result<User, _> = bson::from_bson(Bson::Document(found_user));
                    match loaded_user_doc {
                        Ok(loaded_user) => ApiResponse::ok(json!(ResponseUser::from_user(&loaded_user))),
                        Err(_) => ApiResponse::internal_err(),
                    }                    
                },
                None => ApiResponse::err(json!(format!("user {} not found",  email))),
            }
        },
        Err(_) => ApiResponse::internal_err(),
    }
}
