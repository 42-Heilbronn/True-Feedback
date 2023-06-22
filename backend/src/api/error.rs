use actix_session::{SessionGetError, SessionInsertError};
use actix_web::{error, HttpResponse};
use derive_more::Display;
use diesel::result::DatabaseErrorKind;
use diesel_async::pooled_connection::bb8::RunError;
use std::{convert::From, num::ParseIntError};

#[derive(Debug, Display)]
pub enum ApiError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "NotFound")]
    NotFound,

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            ApiError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ApiError::NotFound => HttpResponse::NotFound().json("NotFound"),
            ApiError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

impl From<RunError> for ApiError {
    fn from(err: RunError) -> ApiError {
        log::error!("{:?}", err);
        ApiError::InternalServerError
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> ApiError {
        log::error!("{:?}", err);
        match err.downcast_ref::<diesel::result::Error>() {
            Some(diesel::result::Error::NotFound) => ApiError::NotFound,
            Some(diesel::result::Error::DatabaseError(kind, info)) => match kind {
                DatabaseErrorKind::UniqueViolation
                | DatabaseErrorKind::ForeignKeyViolation
                | DatabaseErrorKind::CheckViolation
                | DatabaseErrorKind::NotNullViolation => {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ApiError::BadRequest(message);
                }
                _ => ApiError::InternalServerError,
            },
            _ => ApiError::InternalServerError,
        }
    }
}

impl From<ParseIntError> for ApiError {
    fn from(_err: ParseIntError) -> Self {
        ApiError::InternalServerError
    }
}

impl From<SessionGetError> for ApiError {
    fn from(_err: SessionGetError) -> Self {
        ApiError::InternalServerError
    }
}

impl From<SessionInsertError> for ApiError {
    fn from(_err: SessionInsertError) -> Self {
        ApiError::InternalServerError
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(_err: diesel::result::Error) -> Self {
        ApiError::InternalServerError
    }
}
