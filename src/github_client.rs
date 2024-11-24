use reqwest::{
    header::{self, ACCEPT, AUTHORIZATION, USER_AGENT},
    Client,
};
use tracing::error;
use tracing::info;

use crate::{
    configs::GithubConfig,
    github_models::{Conclusion, RunName, WorkflowRunsResponse},
    metric_models::Metric,
};

pub struct GithubApiClient {
    github_url: String,
    owner: String,
    repo: String,
    client: Client,
}

impl GithubApiClient {
    pub fn new() -> Self {
        let config = GithubConfig::new().unwrap();
        let auth = format!("Bearer {}", config.github_token);
        let mut headers = header::HeaderMap::new();
        headers.insert(
            ACCEPT,
            header::HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(
            AUTHORIZATION,
            header::HeaderValue::from_str(&auth).expect("failed to auth"),
        );
        headers.insert(
            "X-GitHub-Api-Version",
            header::HeaderValue::from_static("2022-11-28"),
        );
        headers.insert(
            USER_AGENT,
            header::HeaderValue::from_static("metric-collector"),
        );
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Client builder failed");
        Self {
            github_url: config.github_url,
            repo: config.repo,
            owner: config.owner,
            client,
        }
    }
    pub async fn collect(&self) {
        let pull_request_success_metrics = self
            .get_workflow_runs(RunName::PullRequest, Conclusion::Success)
            .await
            .expect("Fetching workflows information failed");
        let release_failure_metrics = self
            .get_workflow_runs(RunName::Release, Conclusion::Failure)
            .await
            .expect("Fetching workflows information failed");
    }
    async fn get_workflow_runs(
        &self,
        name: RunName,
        conclusion: Conclusion,
    ) -> Result<Vec<Metric>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}repos/{}/{}/actions/runs",
            &self.github_url, self.owner, self.repo
        );
        let resp = self
            .client
            .get(&url)
            .query(&[("status", conclusion.as_str()), ("name", name.as_str())])
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send request: {}", e);
                e
            })?;
        match resp.status() {
            reqwest::StatusCode::OK => {
                let workflow_runs_response: WorkflowRunsResponse = resp.json().await?;
                let runs = &workflow_runs_response.workflow_runs;
                let metrics: Vec<Metric> = runs
                    .iter()
                    .filter_map(|run| {
                        run.run_started_at.map(|started_at| {
                            let duration = run.updated_at - started_at;
                            let hours = duration.num_hours();
                            let minutes = duration.num_minutes();
                            let seconds = duration.num_seconds() % 60;
                            Metric {
                                project_name: self.repo.clone(),
                                result: conclusion.as_str().to_owned(),
                                workflow_id: run.id,
                                workflow_name: name.as_str().to_owned(),
                                duration: format!("{}.{}.{}", hours, minutes, seconds),
                            }
                        })
                    })
                    .collect();
                info!("Collected {:?} metrics", metrics.len());
                info!(
                    "Successfully fetched {:?} {:?} workflow runs with status {:?}",
                    workflow_runs_response.total_count,
                    name.as_str(),
                    conclusion.as_str()
                );
                Ok(metrics)
            }
            status => {
                let error_body = resp.text().await?;
                Err(format!("GitHub API error: {} - {}", status, error_body).into())
            }
        }
    }
}
