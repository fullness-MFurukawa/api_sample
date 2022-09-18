use std::future::Future;
use std::pin::Pin;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use chrono::Duration;
use serde::{Deserialize, Serialize};
use app_commons::application::transfers::UserDto;
use app_commons::presentation::jwt::{ClaimsGenerator, JwtDecoder ,JwtEncoder , JWT_HEADER_KEY};
use crate::error::ApiErrorInfo;
use crate::{Result,ApiAppError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimsResponse {
    pub state: String,
    pub token: String,
}
impl ClaimsResponse {
    pub fn new(_state: &str, _token: &str) -> Self {
        Self {
            state: String::from(_state),
            token: String::from(_token),
        }
    }
}

// クレーム(認証に必要な情報)
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiClaims {
    iat:        i64,    //  Token取得日時
    exp:        i64,    //  Tokenの有効期限
    sub:        String, //  リソースオーナーの識別子
    user_id:    String, //  ユーザーId(Uuid)
    user_name:  String, //  ユーザー名
}
impl ClaimsGenerator<UserDto> for ApiClaims {
    fn generate(user: &UserDto) -> Self {
        let now = chrono::Utc::now();
        let _iat = now.timestamp();
        // クレーム(Payload)の生成
        Self {
            iat: _iat,                                     // 取得日時の設定
            exp: (now + Duration::minutes(5)).timestamp(), // 有効期限を5分に設定
            sub: String::from("M.Furukawa"),            // オーナー識別子を設定
            user_id: user.user_id.clone(),                 // ユーザーidを設定
            user_name: user.user_name.clone(),             // ユーザー名
        }
    }
}
///
/// API用Jwtトークンのエンコードとデコード
///
#[derive(Default)]
pub struct ApiJwt;
// トークンのエンコード デフォルト実装をそのまま利用する
impl JwtEncoder for ApiJwt{}
// トークンのデコード
impl JwtDecoder<ApiClaims, ApiAppError, HttpRequest> for ApiJwt {
    fn decode_header(&self, request: &HttpRequest) -> Result<String> {
        // 認可情報ヘッダーの取得
        let header_value = match request.headers().get(JWT_HEADER_KEY) {
            Some(header) => header,
            None => return Err(ApiAppError::NotAuthorizeError(ApiErrorInfo::new(
                    "authorization error", "Authorization header not found.")))
        };
        // トークンの取得
        let token = header_value.to_str().unwrap();
        let mut split_token = token.split_whitespace();
        // スキーマの取得
        match split_token.next() {
            Some(schema_type) => {
                if schema_type != "Bearer" {
                    return Err(ApiAppError::NotAuthorizeError(ApiErrorInfo::new(
                        "authorization error", "invalid schema type.")));
                }
            }
            None => return Err(ApiAppError::NotAuthorizeError(ApiErrorInfo::new(
                    "authorization error", "invalid schema type.")))
        };
        // JWTトークンの取得
        match split_token.next() {
            Some(jwt_token) => Ok(jwt_token.to_string()),
            None => Err(ApiAppError::NotAuthorizeError(ApiErrorInfo::new(
                "authorization error", "JWT token not found."))),
        }
    }
}

///
/// リクエスト受信時の前処理
///
impl FromRequest for ApiClaims {
    type Error = ApiAppError;
    type Future = Pin<Box<dyn Future<Output = Result<Self>>>>;
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let request = req.clone();
        Box::pin(async move {
            let decoder = ApiJwt::default();
            let token = decoder.decode_header(&request)?;
            match decoder.decode_jwt_token(token.as_str()) {
                Ok(token_data) => Ok(token_data.claims),
                Err(error) => Err(ApiAppError::NotAuthorizeError(ApiErrorInfo::new(
                    "authorization error", error.to_string().as_str()))),
            }
        })
    }
}

