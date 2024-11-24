use dotenv::dotenv;
use github_client::GithubApiClient;
use std::io::Write;
use std::{
    error::Error,
    fs::{self, File},
    path::Path,
};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
mod configs;
mod github_client;
mod github_models;
mod helpers;
mod metric_models;

async fn run() -> Result<(), Box<dyn Error>> {
    info!("Starting application");
    let github_api_client = GithubApiClient::new();
    let github_metrics = github_api_client.collect().await;
    let dir_path = Path::new("output");
    let file_path = dir_path.join("metrics.txt");

    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
    }
    let mut file = File::create(file_path)?;
    for metric in github_metrics {
        writeln!(file, "{:?}", metric)?;
    }

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
