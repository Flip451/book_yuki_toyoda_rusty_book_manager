use adapter::database::connect_database_with;
use axum::Router;
use registry::AppRegistry;
use shared::{config::AppConfig, env::Environment};
use std::net::{Ipv4Addr, SocketAddr};
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use anyhow::{Context, Result};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
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
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .with_state(registry);

    // TCP リスナーの設定
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);

    println!("Listening on {}", addr);

    // サーバーの起動
    axum::serve(listener, app)
        .await
        .context("Unexpected server error")
        .inspect_err(|e| {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "Unexpected error"
            )
        })
}

fn init_logger() -> Result<()> {
    let log_level = match shared::env::which() {
        Environment::Development => "debug",
        Environment::Production => "info",
    };

    // ログレベルの設定
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    // ログの出力形式を設定
    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()?;

    Ok(())
}
