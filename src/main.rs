use actix_web::{middleware, web, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use app_commons::application::sea_orm::provider::AppServiceProvider;
use app_commons::infrastructure::pool::PoolProvider;
use app_commons::infrastructure::sea_orm::pool_impl::SeaOrmPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ロガーの初期化
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // SeaOrmのDatabaseConnectionを取得
    let pool = SeaOrmPool::get().await;
    // アプリケーションサービスプロバイダの生成
    let provider = AppServiceProvider::new();

    /*  サーバーの実行 */
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default()) // ロギングミドルウェアの登録
            // DatabaseConnectionの登録
            .app_data(web::Data::new(pool.clone()))
            // アプリケーションサービスプロバイダの登録
            .app_data(web::Data::new(provider.clone()))
            // サービスの登録
            .configure(set_config)
    })
        .bind_openssl("127.0.0.1:8082", create_ssl_acceptor_builder())?
        .run()
        .await
}

///
/// OpenSSL SslAcceptorBuilderの生成
///
fn create_ssl_acceptor_builder() -> SslAcceptorBuilder {
    // OpenSSL構造を管理し、暗号スイート、セッションオプションなどを構成する
    let mut builder: SslAcceptorBuilder =
        SslAcceptor::mozilla_intermediate_v5(SslMethod::tls_server()).unwrap();
    // 秘密鍵の設定
    builder
        .set_private_key_file("localhost-key.pem", SslFiletype::PEM)
        .unwrap();
    // 証明書の設定
    builder.set_certificate_chain_file("localhost.pem").unwrap();
    builder
}

///
/// サービスの設定
///
pub fn set_config(config: &mut web::ServiceConfig) {
    use api_sample::handler::authenticate::AuthenticateHandler;
    use api_sample::handler::product_register::ProductRegisterHandler;
    use api_sample::handler::product_search::ProductSearchHandler;
    config.service(
        web::scope("/api_sample")
            .route("/", web::post().to(AuthenticateHandler::authenticate))
            .route(
                "/search/product",
                web::get().to(ProductSearchHandler::search),
            )
            .route(
                "/register/product",
                web::post().to(ProductRegisterHandler::register),
            ),
    );
}
