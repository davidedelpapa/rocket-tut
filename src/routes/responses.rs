use rocket::*;
use rocket::response;
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket_contrib::json::JsonValue;
use rocket_contrib::json;

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
