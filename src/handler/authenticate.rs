use sea_orm::DatabaseConnection;
use std::sync::Arc;
use actix_web::{web, HttpResponse, Responder};
use mime::APPLICATION_JSON;
use app_commons::application::sea_orm::provider::AppServiceProvider;
use app_commons::presentation::forms::LoginForm;
use app_commons::presentation::jwt::{ClaimsGenerator, JwtEncoder};
use app_commons::presentation::validate::AppValidator;
use crate::error::ApiErrorInfo;
use crate::jwt::{ApiClaims, ApiJwt, ClaimsResponse};
use crate::{ApiAppError, Result};
///
/// 認証 リクエストハンドラ
///
pub struct AuthenticateHandler;
impl AuthenticateHandler {
    // ログイン認証
    pub async fn authenticate(
        form: web::Json<LoginForm>,
        pool: web::Data<Arc<DatabaseConnection>>,
        provider: web::Data<Arc<AppServiceProvider>>) -> Result<impl Responder> {
        // 入力値の検証
        match form.validate_value() {
            Ok(_) => (),
            Err(error) => {
                return Ok(HttpResponse::BadRequest()
                    .content_type(APPLICATION_JSON)
                    .json(error.errors))
            }
        };
        // 認証処理
        match provider.authenticate_service.execute(&pool, &form).await {
            Ok(user) => {
                // JWTトークンの生成
                let claims = ApiClaims::generate(&user);
                let token = ApiJwt::encode(&claims);
                Ok(HttpResponse::Ok()
                    .content_type(APPLICATION_JSON)
                    .json(ClaimsResponse::new("authenticate success", token.as_str())))
            }
            Err(error) => {
                let message = ApiAppError::from(error)?;
                Err(ApiAppError::SearchError(ApiErrorInfo::new(
                    "authenticate error", message.as_str())))
            }
        }
    }
}
