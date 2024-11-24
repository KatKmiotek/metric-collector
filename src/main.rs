use std::error::Error;
use dotenv::dotenv;
use github_client::GithubApiClient;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
mod github_client;
mod github_models;
mod configs;

async fn run() -> Result<(), Box<dyn Error>> {
    info!("Starting application");
    let github_api_client = GithubApiClient::new();
    github_api_client.collect().await;
        
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        // .with_max_level(Level::DEBUG)
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
