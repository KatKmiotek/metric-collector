use std::error::Error;

use actix_web::{get, http::header::ContentType, HttpResponse};

use crate::{github_client::GithubApiClient, utils::SaveData};

#[get("/metrics")]
pub async fn get_metrics() -> Result<HttpResponse, Box<dyn Error>> {
    let github_api_client = GithubApiClient::new();
    let github_metrics = github_api_client.collect().await;
    SaveData::save_to_file(&github_metrics).expect("Saving to file failed");
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(&github_metrics))
}
