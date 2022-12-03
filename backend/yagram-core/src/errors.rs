use actix_web::http::StatusCode;
use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use std::fmt::{Display, Formatter};

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal server Error")]
    InternalServerError,
    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),
    #[display(fmt = "JWKSFetchError")]
    JWKSFetchError,
    #[display(fmt = "Not found")]
    NotFound,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError()
                .json("Internal Server Error, Please try again later"),
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::JWKSFetchError => {
                HttpResponse::InternalServerError().json("Could not fetch JWKS")
            }

            ServiceError::NotFound => HttpResponse::NotFound().json("Not found"),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServiceError::JWKSFetchError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
