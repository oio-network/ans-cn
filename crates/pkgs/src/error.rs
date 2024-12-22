use crate::Json;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use reqwest::Error as ReqwestError;
use sea_orm::error::DbErr;
use serde::Serialize;
use serde_json::Error as SerdeJsonError;
use worker::{kv::KvError, Error as WorkerError};

#[derive(Debug)]
pub enum Error {
    JsonRejection(JsonRejection),
    SerdeJsonError(SerdeJsonError),
    WorkerError(WorkerError),
    KvError(KvError),
    DbErr(DbErr),
    ReqwestError(ReqwestError),
    InternalServerError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            code: u16,
            message: String,
        }

        let (code, message) = match self {
            Error::JsonRejection(rejection) => (rejection.status(), rejection.body_text()),
            Error::SerdeJsonError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            Error::WorkerError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            Error::KvError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            Error::DbErr(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            Error::ReqwestError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            Error::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
        };

        Json(ErrorResponse {
            code: code.as_u16(),
            message,
        })
        .with_status_code(code)
        .into_response()
    }
}

impl From<JsonRejection> for Error {
    fn from(value: JsonRejection) -> Self {
        Self::JsonRejection(value)
    }
}

impl From<SerdeJsonError> for Error {
    fn from(value: SerdeJsonError) -> Self {
        Self::SerdeJsonError(value)
    }
}

impl From<WorkerError> for Error {
    fn from(value: WorkerError) -> Self {
        Self::WorkerError(value)
    }
}

impl From<KvError> for Error {
    fn from(value: KvError) -> Self {
        Self::KvError(value)
    }
}

impl From<DbErr> for Error {
    fn from(value: DbErr) -> Self {
        Self::DbErr(value)
    }
}

impl From<ReqwestError> for Error {
    fn from(value: ReqwestError) -> Self {
        Self::ReqwestError(value)
    }
}
