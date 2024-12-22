use adapter::{database::connect_database_with, redis::RedisClient};
use axum::{http::Method, Router};
use opentelemetry::global;
use registry::AppRegistryImpl;
use shared::{config::AppConfig, env::Environment};
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tower_http::{
    cors::{self, CorsLayer},
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

    // Redis への接続を担うクライアントのインスタンス化
    let kvs = Arc::new(RedisClient::new(&app_config.redis)?);

    // 依存解決
    let registry = Arc::new(AppRegistryImpl::new(pool, kvs, app_config));

    // ルーティングの設定
    let app = Router::new()
        .merge(api::route::v1::routes())
        .merge(api::route::auth::build_auth_routers())
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
        .layer(cors())
        .with_state(registry);

    // TCP リスナーの設定
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);

    println!("Listening on {}", addr);

    // サーバーの起動
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
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

    // 環境変数の読み込み
    let host = std::env::var("JAEGER_HOST")?;
    let port = std::env::var("JAEGER_PORT")?;
    let endpoint = format!("{}:{}", host, port);

    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_endpoint(endpoint)
        .with_service_name("book-manager")
        .with_auto_split_batch(true)
        .with_max_packet_size(8192)
        .install_simple()?;

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // ログレベルの設定
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    // ログの出力形式を設定
    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .json();

    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .with(opentelemetry)
        .try_init()?;

    Ok(())
}

fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_headers(cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(cors::Any)
}

async fn shutdown_signal() {
    fn purge_spans() {
        global::shutdown_tracer_provider();
    }

    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM signal handler")
            .recv()
            .await
            .expect("Failed to receive SIGTERM signal");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Ctrl-C を受信しました。");
            purge_spans();
        },
        _ = terminate => {
            tracing::info!("SIGTERM を受信しました。");
            purge_spans();
        },
    }
}
