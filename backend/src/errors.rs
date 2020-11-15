use actix_web::{error::BlockingError, HttpResponse, ResponseError};
use derive_more::Display;
use diesel::result::Error as DieselError;
use r2d2::Error as R2D2Error;
use riven::RiotApiError;

#[derive(Debug, Display)]
pub enum ServiceError {
    // #[display(fmt = "Internal Server Error")]
    // InternalServerError,

    // #[display(fmt = "BadRequest: {}", _0)]
    // BadRequest(String),
    #[display(fmt = "JWKSFetchError: {}", _0)]
    JWKSFetchError(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            // ServiceError::InternalServerError => HttpResponse::InternalServerError()
            //     .json("Internal Server Error, Please try again later"),
            // ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::JWKSFetchError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
        }
    }
}

#[derive(Debug, Display)]
pub enum UpdateError {
    #[display(fmt = "QueryError")]
    QueryError(DieselError),
    #[display(fmt = "PoolError")]
    PoolError(R2D2Error),
    #[display(fmt = "BlockError")]
    BlockError(BlockingError<DieselError>),
    #[display(fmt = "RiotError")]
    RiotError(RiotApiError),
}
