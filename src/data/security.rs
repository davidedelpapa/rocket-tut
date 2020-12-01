use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use anyhow::Result as AnyResult;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use jsonwebtoken::errors::ErrorKind;

const SECRET: &str = "secret297152aebda7"; // Change this to whatever your own

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Claims {
    #[serde(with = "jwt_numeric_date")]
    exp: DateTime<Utc>,
    id: String,
}
impl Claims {
    pub fn new(exp: DateTime<Utc>, id: String) -> Self {
        // Normalize to UNIX timestamps
        let exp = exp.date().and_hms_milli(exp.hour(), exp.minute(), exp.second(), 0);
        Self {
            exp,
            id,
        }
    }
}
mod jwt_numeric_date {
    //! Custom serialization of DateTime<Utc> to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    /// Serializes a DateTime<Utc> to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
            .single() // If there are multiple or no valid DateTimes from timestamp, return None
            .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}

pub fn sign_token(user_id: String) -> AnyResult<String> {
    // Expires in 1 day: set in .env or adapt
    let exp = Utc::now() + chrono::Duration::days(1);
    let claims = Claims::new(exp, user_id);

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )?;
    Ok(token)
}

#[derive(Debug, PartialEq)]
pub enum JwtDecodeError {
    Expired, // The token has expired. We could match to redirect to a login
    Generic, // Other unforseen errors
}

pub fn decode_token(token: String) ->Result<String, JwtDecodeError> {
    match jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::default(),
    ){
        Ok(token_data) => Ok(token_data.claims.id),
        Err(err) => match *err.kind() {
            ErrorKind::ExpiredSignature => Err(JwtDecodeError::Expired),
            _ => Err(JwtDecodeError::Generic),
        },
    }
}

pub struct JwtGuard(String);

#[derive(Debug, PartialEq)]
pub enum JwtGuardError {
    Missing,
    TokenError(JwtDecodeError),
}

impl<'a, 'r> FromRequest<'a, 'r> for JwtGuard {
    type Error = JwtGuardError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        let tokens = cookies.get("t");
        match tokens {
            None => Outcome::Failure((Status::BadRequest, JwtGuardError::Missing)),
            Some(t) => {
                match decode_token(t.value().to_string()){
                    Ok(id) => Outcome::Success(JwtGuard(id)),
                    Err(JwtDecodeError::Expired) => Outcome::Failure((Status::BadRequest, JwtGuardError::TokenError(JwtDecodeError::Expired))),
                    Err(JwtDecodeError::Generic) => Outcome::Failure((Status::BadRequest, JwtGuardError::TokenError(JwtDecodeError::Generic))),
                }
            },
        }
    }
}
