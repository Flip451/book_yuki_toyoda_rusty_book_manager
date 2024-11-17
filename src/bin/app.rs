use adapter::database::connect_database_with;
use axum::Router;
use registry::AppRegistry;
use shared::config::AppConfig;
use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap().await
}
async fn bootstrap() -> Result<()> {
    // 環境変数からアプリケーション全体の設定を読み込む
    let app_config = AppConfig::new()?;

    // データベースの接続
    let pool = connect_database_with(&app_config.database);

    // 依存解決
    let registry = AppRegistry::new(pool);

    // ルーティングの設定
    let app = Router::new()
        .merge(api::route::health::build_health_check_routers())
        .merge(api::route::book::build_book_routers())
        .with_state(registry);

    // TCP リスナーの設定
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    // サーバーの起動
    Ok(axum::serve(listener, app).await?)
}
