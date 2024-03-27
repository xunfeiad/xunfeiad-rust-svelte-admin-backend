use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use derive_more::{Display, Error};
use hmac::digest::InvalidLength;
use jwt::Error;
use sea_orm::{DatabaseConnection, DbErr};
use serde::Serialize;
use std::convert::From;
use std::num::ParseIntError;
use std::result::Result;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use validator::ValidationErrorsKind;

pub mod crypt;

#[derive(Debug)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub fn get_current_datetime() -> sea_orm::prelude::DateTime {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Fetch system time failed.")
        .as_millis();
    sea_orm::prelude::DateTime::from_timestamp_millis(timestamp as i64)
        .expect("Parse system time failed.")
}

pub type WebResult<T> = Result<T, WebError>;

#[derive(Debug, Display, Error)]
pub enum WebError {
    #[display(fmt = "Internal Server Error:{}", msg)]
    InternalError { msg: String },

    #[display(fmt = "Bad Request Error:{}", msg)]
    BadClientData { msg: String },

    #[display(fmt = "Timeout Error:{}", msg)]
    Timeout { msg: String },
    #[display(fmt = "SeaOrm Error:{}", msg)]
    SeaORMError { msg: String },
    #[display(fmt = "Invalidate Data Error: {}", msg)]
    InvalidateDataError { msg: String },
    #[display(fmt = "Invalidate JWT Error: {}", msg)]
    InvalidateJWTError { msg: String },
}

impl error::ResponseError for WebError {
    fn status_code(&self) -> StatusCode {
        match self {
            WebError::InternalError { msg } | WebError::SeaORMError { msg } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            WebError::BadClientData { msg } => StatusCode::BAD_REQUEST,
            WebError::Timeout { msg } => StatusCode::GATEWAY_TIMEOUT,
            WebError::InvalidateDataError { msg } => StatusCode::BAD_REQUEST,
            WebError::InvalidateJWTError { msg } => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(
                ResponseErrors {
                    code: self.status_code().as_u16(),
                    detail: self.to_string(),
                }
                .format(),
            )
    }
}

#[derive(Serialize)]
pub struct ResponseErrors {
    code: u16,
    detail: String,
}

impl ResponseErrors {
    fn format(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl From<DbErr> for WebError {
    fn from(value: DbErr) -> Self {
        WebError::InternalError {
            msg: value.to_string(),
        }
    }
}

impl From<anyhow::Error> for WebError {
    fn from(value: anyhow::Error) -> Self {
        Self::InternalError {
            msg: value.to_string(),
        }
    }
}
