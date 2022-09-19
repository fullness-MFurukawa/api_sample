use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use log::error;
use mime::APPLICATION_JSON;
use app_commons::error::AppError;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;

// エラー情報構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorInfo {
    pub status:  String,  // 状態
    pub message: String,  // メッセージ
}
impl ApiErrorInfo {
    pub fn new(_status: &str, _message: &str) -> Self {
        Self {status:_status.to_owned(), message:_message.to_owned()}
    }
}
///
/// プレゼンテーションエラー API
///
#[derive(Debug, Error)]
pub enum ApiAppError {
    NotAuthorizeError(ApiErrorInfo) , // 未認可エラー
    AuthenticateError(ApiErrorInfo) , // 認証エラー
    SearchError(ApiErrorInfo)   ,     // 検索エラー
    RegisterError(ApiErrorInfo) ,     // 登録エラー
    InternalError(AppError)      // 内部エラー
}
impl Display for ApiAppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<AppError> for ApiAppError {
    fn from(error: AppError) -> Self {
        match error {
            AppError::SearchError(msg) =>
                Self::SearchError(ApiErrorInfo::new("search error" , msg.as_str())),
            AppError::RegisterError(msg) =>
                Self::RegisterError(ApiErrorInfo::new("register error" , msg.as_str())),
            AppError::AuthenticateError(msg) =>
                Self::AuthenticateError(ApiErrorInfo::new("authenticate error" , msg.as_str())),
            AppError::InternalError(..) => ApiAppError::InternalError(error)
        }
    }
}
///
/// エラーレスポンスの生成
///
impl ResponseError for ApiAppError {
    // ステータスの設定
    fn status_code(&self) -> StatusCode {
        match self {
            ApiAppError::InternalError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiAppError::AuthenticateError(..) => StatusCode::BAD_REQUEST,
            ApiAppError::RegisterError(..) => StatusCode::BAD_REQUEST,
            ApiAppError::SearchError(..) => StatusCode::NOT_FOUND,
            ApiAppError::NotAuthorizeError(..) => StatusCode::UNAUTHORIZED,
        }
    }
    // エラーレスポンスの生成
    fn error_response(&self) -> HttpResponse {
        match self {
            // 内部エラー
            ApiAppError::InternalError(error) => {
                error!("{:?}", error); // エラーログを出力する
                // エラー情報を生成する
                let info = ApiErrorInfo::new("stop service", "Service is down.");
                HttpResponse::InternalServerError().content_type(APPLICATION_JSON).json(info)
            },
            // 検索エラー
            ApiAppError::SearchError(info) =>
                HttpResponse::NotFound().content_type(APPLICATION_JSON).json(info),
            // 登録　、認証エラー
            ApiAppError::RegisterError(info) |
            ApiAppError::AuthenticateError(info) =>
                HttpResponse::BadRequest().content_type(APPLICATION_JSON).json(info),
            // 無認可エラー
            ApiAppError::NotAuthorizeError(info) =>
                HttpResponse::Unauthorized().content_type(APPLICATION_JSON).json(info)
        }
    }
}
