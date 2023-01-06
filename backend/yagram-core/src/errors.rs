use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde::{Deserialize, Serialize, Serializer};
use serde_json::json;
use strum_macros::Display;

#[derive(Debug, Display, Serialize)]
pub enum ServiceError {
    InternalServerError,
    BadRequest(String),
    NotFound,
    AuthenticationError(String),
    JWKSFetchError,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(format!(
                "{{ \"status\": {}, \"message\": {} }}",
                self.status_code().as_u16(),
                json!(self)
            ))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::InternalServerError | ServiceError::JWKSFetchError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ServiceError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServiceError::NotFound => StatusCode::NOT_FOUND,
            ServiceError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
        }
    }
}
