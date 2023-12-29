use log::info;
use crate::module::app_module;
use dotenv;
use env_logger;
use crate::config::{app_configuration};

mod user;

mod todo;
mod module;
mod auth;
mod config;
mod error;
mod response;

async fn bootstrap_api() {
    let app_config = app_configuration();
    let routes = app_module(&app_config).await;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &app_config.port)).await.unwrap();
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();

    info!("Server stopped.");
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    info!("🚀 Server starting...");
    bootstrap_api().await;
}
