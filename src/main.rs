use std::{process::exit, sync::Arc};

use axum::http::{
    Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::filter::LevelFilter;

use crate::{config::Config, db::DBClient, routers::create_router};

mod config;
mod db;
mod error;
mod middleware;
mod models;
mod repositories;
mod routers;
mod services;
mod utils;

#[derive(Debug, Clone)]
pub struct AppState {
    config: Config,
    db_client: DBClient,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    dotenv().ok();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Conectado ao banco de dados!");
            pool
        }
        Err(err) => {
            println!("🔥 Não foi possível conectar ao DB: {:?}", err);
            exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ]);

    let db_client = DBClient::new(pool);

    let app_state = AppState {
        config: config,
        db_client: db_client,
    };

    let app = create_router(Arc::new(app_state.clone())).layer(cors.clone());

    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &app_state.config.server_port))
            .await
            .unwrap();

    println!(
        "✅ Servidor conectado na porta {}",
        &app_state.config.server_port
    );

    axum::serve(listener, app).await.unwrap();
}
