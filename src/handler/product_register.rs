use sea_orm::DatabaseConnection;
use std::sync::Arc;
use actix_web::{web, HttpResponse, Responder};
use mime::APPLICATION_JSON;
use app_commons::application::sea_orm::provider::AppServiceProvider;
use app_commons::presentation::forms::ProductRegisterForm;
use app_commons::presentation::validate::AppValidator;
use crate::error::ApiErrorInfo;
use crate::jwt::ApiClaims;
use crate::{ApiAppError, Result};

///
/// 商品登録 リクエストハンドラ
///
pub struct ProductRegisterHandler;
impl ProductRegisterHandler {
    pub async fn register(
        _claims: ApiClaims,
        form: web::Json<ProductRegisterForm>,
        pool: web::Data<Arc<DatabaseConnection>>,
        provider: web::Data<Arc<AppServiceProvider>>) -> Result<impl Responder> {
        // 入力値の検証
        match form.validate_value() {
            Err(error) => {
                return Ok(HttpResponse::BadRequest()
                    .content_type(APPLICATION_JSON).json(error.errors))
            },Ok(_) => ()
        };
        // 商品を永続化する
        match provider.register_service.execute(&pool, &form).await {
            Ok(new_product) => Ok(HttpResponse::Ok()
                .content_type(APPLICATION_JSON).json(new_product)),
            Err(error) => {
                let message = ApiAppError::from(error)?;
                Err(ApiAppError::SearchError(ApiErrorInfo::new(
                    "register error", message.as_str())))
            }
        }
    }
}
