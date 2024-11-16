use anyhow::Result;
use sqlx::{postgres::PgConnectOptions, PgPool};
use std::net::{Ipv4Addr, SocketAddr};

use axum::{extract::State, http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    // データベースの設定
    let database_config = DatabaseConfig {
        host: "localhost".to_string(),
        port: 5432,
        username: "app".to_string(),
        password: "passwd".to_string(),
        database: "app".to_string(),
    };
    let conn_pool = connect_database_with(database_config);

    // ルーターの設定（ハンドラの登録）
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/health/db", get(health_check_db))
        // ルーターの `State` にプールを登録しておき、各ハンドラで使えるようにする
        .with_state(conn_pool);

    // TCP リスナーの設定
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    // サーバーの起動
    Ok(axum::serve(listener, app).await?)
}

// ハンドラ
async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tokio::test]
async fn health_check_works() {
    let status_code = health_check().await;
    assert_eq!(status_code, StatusCode::OK);
}

struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl From<DatabaseConfig> for PgConnectOptions {
    fn from(config: DatabaseConfig) -> Self {
        Self::new()
            .host(&config.host)
            .port(config.port)
            .username(&config.username)
            .password(&config.password)
            .database(&config.database)
    }
}

fn connect_database_with(config: DatabaseConfig) -> PgPool {
    PgPool::connect_lazy_with(config.into())
}

async fn health_check_db(State(db): State<PgPool>) -> StatusCode {
    let connection_result = sqlx::query("SELECT 1").fetch_one(&db).await;
    match connection_result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[sqlx::test]
async fn health_check_db_works(pool: PgPool) {
    let status_code = health_check_db(State(pool)).await;
    assert_eq!(status_code, StatusCode::OK);
}
