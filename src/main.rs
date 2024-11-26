use actix_web::{App, HttpServer};
use controllers::get_metrics;
use dotenv::dotenv;
use std::error::Error;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
mod configs;
mod controllers;
mod github_client;
mod github_models;
mod helpers;
mod metric_models;
mod utils;

async fn run() -> Result<(), Box<dyn Error>> {
    info!("Starting application");
    let _ = HttpServer::new(|| App::new().service(get_metrics))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .pretty()
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting up default subscriber failed");
    run().await?;
    Ok(())
}
