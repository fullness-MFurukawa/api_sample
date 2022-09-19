use crate::Result;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use log::error;
use mime::APPLICATION_JSON;
use app_commons::error::AppError;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorInfo {
    pub state: String,
    pub message: String,
}
impl ApiErrorInfo {
    pub fn new(_state: &str, _message: &str) -> Self {
        Self {
            state: String::from(_state),
            message: String::from(_message),
        }
    }
}
///
/// プレゼンテーションエラー API
///
#[derive(Debug, Error)]
pub enum ApiAppError {
    NotAuthorizeError(ApiErrorInfo),
    AuthenticateError(ApiErrorInfo),
    SearchError(ApiErrorInfo),
    InternalError(anyhow::Error),
}
impl Display for ApiAppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl ApiAppError {
    pub fn from(error: AppError) -> Result<String> {
        let msg = match error {
            AppError::SearchError(msg)
            | AppError::RegisterError(msg)
            | AppError::AuthenticateError(msg) => msg,
            AppError::InternalError(error) => return Err(ApiAppError::InternalError(error)),
        };
        Ok(msg)
    }
}
impl ResponseError for ApiAppError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiAppError::InternalError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiAppError::AuthenticateError(..) => StatusCode::BAD_REQUEST,
            ApiAppError::SearchError(..) => StatusCode::NOT_FOUND,
            ApiAppError::NotAuthorizeError(..) => StatusCode::UNAUTHORIZED,
        }
    }
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiAppError::InternalError(error) => {
                error!("{:?}", error);
                let info = ApiErrorInfo::new("stop service", "Service is down.");
                HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON).json(info)
            }
            ApiAppError::SearchError(info) | ApiAppError::AuthenticateError(info) => {
                HttpResponse::BadRequest()
                    .content_type(APPLICATION_JSON).json(info)
            }
            ApiAppError::NotAuthorizeError(info) =>
                HttpResponse::Unauthorized()
                .content_type(APPLICATION_JSON).json(info)
        }
    }
}
