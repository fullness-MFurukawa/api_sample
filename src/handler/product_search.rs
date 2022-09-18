use sea_orm::DatabaseConnection;
use std::sync::Arc;
use actix_web::{web, HttpResponse, Responder};
use mime::APPLICATION_JSON;
use app_commons::application::sea_orm::provider::AppServiceProvider;
use app_commons::presentation::forms::ProductSearchForm;
use app_commons::presentation::validate::AppValidator;
use crate::handler::error::ApiErrorInfo;
use crate::handler::jwt::ApiClaims;
use crate::{ApiAppError, Result};

///
/// 商品検索 リクエストハンドラ
///
pub struct ProductSearchHandler;
impl ProductSearchHandler {
    pub async fn search(
        _claims: ApiClaims,
        form: web::Json<ProductSearchForm>,
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

        match provider.search_service.search(&pool, &form).await {
            Ok(products) => Ok(HttpResponse::Ok()
                .content_type(APPLICATION_JSON)
                .json(products)),
            Err(error) => {
                let message = ApiAppError::from(error)?;
                Err(ApiAppError::SearchError(ApiErrorInfo::new(
                    "search error", message.as_str())))
            }
        }
    }
}
