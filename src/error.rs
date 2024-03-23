use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum NotifluxErrorType {
    EnvError,
    IOError,
    Error,
    ValidationError,
    Base64DecodeError,
    JWTDecodeError,
}

#[derive(Debug, Eq, PartialEq)]
pub struct NotifluxError {
    pub message: Option<String>,
    pub error_type: NotifluxErrorType,
}

#[derive(Serialize)]
pub struct NotifluxErrorResponse {
    pub error: String,
}

impl fmt::Display for NotifluxErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for NotifluxErrorType {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }
}

impl fmt::Display for NotifluxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NotifluxError: {:?}", self.error_type)
    }
}

impl ResponseError for NotifluxError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            NotifluxErrorType::EnvError
            | NotifluxErrorType::Error
            | NotifluxErrorType::IOError
            | NotifluxErrorType::Base64DecodeError
            | NotifluxErrorType::JWTDecodeError => StatusCode::INTERNAL_SERVER_ERROR,
            NotifluxErrorType::ValidationError => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(NotifluxErrorResponse {
            error: self.message(),
        })
    }
}

impl NotifluxError {
    fn message(&self) -> String {
        match self {
            NotifluxError {
                message: Some(message),
                error_type: _,
            } => message.clone(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
}

impl From<std::env::VarError> for NotifluxError {
    fn from(error: std::env::VarError) -> Self {
        log::error!("Error parsing environment variable: {}", error);
        NotifluxError {
            message: Some("Unexpected error parsing environment variable".to_string()),
            error_type: NotifluxErrorType::EnvError,
        }
    }
}

impl From<std::num::ParseIntError> for NotifluxError {
    fn from(error: std::num::ParseIntError) -> Self {
        log::error!("Error parsing integer: {}", error);
        NotifluxError {
            message: Some("Unexpected error parsing integer".to_string()),
            error_type: NotifluxErrorType::Error,
        }
    }
}

impl From<std::io::Error> for NotifluxError {
    fn from(error: std::io::Error) -> Self {
        log::error!("IO Error: {}", error);
        NotifluxError {
            message: Some("Unexpected IO error".to_string()),
            error_type: NotifluxErrorType::IOError,
        }
    }
}

impl From<base64::DecodeError> for NotifluxError {
    fn from(error: base64::DecodeError) -> Self {
        log::error!("Base64 decode error: {}", error);
        NotifluxError {
            message: Some("Unexpected base64 decode error".to_string()),
            error_type: NotifluxErrorType::Base64DecodeError,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for NotifluxError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        log::error!("JWT decode error: {}", error);
        NotifluxError {
            message: Some("Unexpected JWT decode error".to_string()),
            error_type: NotifluxErrorType::JWTDecodeError,
        }
    }
}

impl From<NotifluxError> for anyhow::Error {
    fn from(error: NotifluxError) -> Self {
        anyhow::Error::msg(error.message())
    }
}
