use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde::Serialize;
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
            .body(format!("{{ \"message\": \"{}\" }}", self))
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
