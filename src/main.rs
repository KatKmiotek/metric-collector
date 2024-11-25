use dotenv::dotenv;
use github_client::GithubApiClient;
use std::error::Error;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use utils::SaveData;
mod configs;
mod github_client;
mod github_models;
mod helpers;
mod metric_models;
mod utils;

async fn run() -> Result<(), Box<dyn Error>> {
    info!("Starting application");
    let github_api_client = GithubApiClient::new();
    let github_metrics = github_api_client.collect().await;
    SaveData::save_to_file(github_metrics).expect("Saving to file failed");

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
