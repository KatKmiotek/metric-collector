use std::{error::Error, path::Path};

use actix_web::{get, http::header::ContentType, HttpResponse};
use serde_json::json;
use tracing::info;

use crate::metric_models::ProjectMetric;

#[get("/metrics")]
pub async fn get_metrics() -> Result<HttpResponse, Box<dyn Error>> {
    let dir_path = Path::new("output");
    let file_path = dir_path.join("metrics.txt");
    let metrics_file_content = tokio::fs::read_to_string(file_path).await?;
    let github_metrics: Vec<ProjectMetric> = serde_json::from_str(&metrics_file_content)?;
    Ok(match github_metrics.len() {
        0 => HttpResponse::BadRequest().json(json!({ "error": "No metrics data available" })),
        _ => {
            info!("Request completed with status 200");
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(&github_metrics)
        }
    })
}
