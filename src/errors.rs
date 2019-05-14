use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

/// ServiceError is used to represent the potential
/// errors that might occur when accessing this API
#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Service Errror")]
    InternalServerError,

    #[display(fmt = "Bad Request")]
    BadRequest(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
        }
    }
}
