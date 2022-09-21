use std::sync::Arc;
use serde::{Deserialize, Serialize};
use sea_orm::DatabaseConnection;
use actix_web::{web, HttpResponse, Responder};
use mime::APPLICATION_JSON;
use app_commons::application::sea_orm::provider::AppServiceProvider;
use app_commons::presentation::forms::LoginForm;
use app_commons::presentation::jwt::{ClaimsGenerator, JwtEncoder};
use app_commons::presentation::validate::AppValidator;
use crate::jwt::{ApiClaims, ApiJwt};
use crate::{ApiAppError, Result};
///
/// 認証 リクエストハンドラ
///
pub struct AuthenticateHandler;
impl AuthenticateHandler {
    // ログイン認証
    pub async fn authenticate(
        form: web::Json<LoginForm>, pool: web::Data<Arc<DatabaseConnection>>,
        provider: web::Data<Arc<AppServiceProvider>>) -> Result<impl Responder> {
        // 入力値の検証
        match form.validate_value() {
            Err(error) => {
                // 検証エラーをクライアントに返す
                return Ok(HttpResponse::BadRequest().content_type(APPLICATION_JSON).json(error.errors))
            },Ok(_) => ()
        };
        // 認証結果レスポンス
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct ClaimsResponse {
            status: String,
            token: String
        }
        // 認証処理
        match provider.authenticate_service.execute(&pool, &form).await {
            Ok(user) => { // 認証成功
                // JWTトークンの生成
                let claims = ApiClaims::generate(&user);
                let token = ApiJwt::encode(&claims);
                Ok(HttpResponse::Ok() // トークンを返す
                    .content_type(APPLICATION_JSON)
                    .json(ClaimsResponse{status:String::from("authenticate success"), token}))
            },
            Err(error) => Err(ApiAppError::from(error)) // エラーハンドにエラー通知する
        }
    }
}
