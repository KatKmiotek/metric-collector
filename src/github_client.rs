use reqwest::{
    header::{self, ACCEPT, AUTHORIZATION, USER_AGENT},
    Client,
};
use tracing::error;
use tracing::info;

use crate::{
    configs::GithubConfig,
    github_models::{Conclusion, PullRequest, RunName, WorkflowRunsResponse},
    helpers::DurationFormatter,
    metric_models::{GitHubMetric, PullRequestMetric, WorkflowMetric},
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

    pub async fn collect(&self) -> Vec<GitHubMetric> {
        let mut github_metrics: Vec<GitHubMetric> = Vec::new();
        let pull_request_success_metrics = self
            .get_workflow_runs_metrics(RunName::PullRequest, Conclusion::Success)
            .await
            .expect("Fetching workflows information failed")
            .into_iter()
            .map(GitHubMetric::Workflow);
        let release_failure_metrics = self
            .get_workflow_runs_metrics(RunName::Release, Conclusion::Failure)
            .await
            .expect("Fetching workflows information failed")
            .into_iter()
            .map(GitHubMetric::Workflow);
        github_metrics.extend(pull_request_success_metrics);
        github_metrics.extend(release_failure_metrics);
        let pr_metrics = self
            .get_pr_metrics()
            .await
            .expect("Fetching PR metrics failed")
            .into_iter()
            .map(GitHubMetric::PullRequest);
        github_metrics.extend(pr_metrics);
        github_metrics
    }

    async fn get_workflow_runs_metrics(
        &self,
        name: RunName,
        conclusion: Conclusion,
    ) -> Result<Vec<WorkflowMetric>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}repos/{}/{}/actions/runs",
            &self.github_url, self.owner, self.repo
        );
        let workflow_runs_response: WorkflowRunsResponse = self
            .make_github_request(
                &url,
                &[("status", conclusion.as_str()), ("name", name.as_str())],
            )
            .await?;
        let runs = &workflow_runs_response.workflow_runs;
        let metrics: Vec<WorkflowMetric> = runs
            .iter()
            .filter_map(|run| {
                run.run_started_at.map(|started_at| {
                    let duration = run.updated_at - started_at;
                    WorkflowMetric {
                        project_name: self.repo.clone(),
                        result: conclusion.as_str().to_owned(),
                        workflow_id: run.id,
                        workflow_name: name.as_str().to_owned(),
                        duration: duration.format_duration(),
                        event: String::from("Workflow"),
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

    async fn get_pr_metrics(&self) -> Result<Vec<PullRequestMetric>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}repos/{}/{}/pulls",
            &self.github_url, self.owner, self.repo
        );
        let pr_response: Vec<PullRequest> = self
            .make_github_request(&url, &[("state", "closed")])
            .await?;
        let pull_requests = &pr_response;
        let metrics: Vec<PullRequestMetric> = pull_requests
            .iter()
            .filter_map(|pr| {
                pr.merged_at.map(|merged_at| {
                    let duration = merged_at - pr.created_at;
                    PullRequestMetric {
                        project_name: self.repo.clone(),
                        pull_request_id: pr.id,
                        duration: duration.format_duration(),
                        event: String::from("PR"),
                    }
                })
            })
            .collect();
        info!("Collected {:?} metrics", metrics.len());
        info!(
            "Successfully fetched {:?} pull requests",
            pull_requests.len(),
        );
        Ok(metrics)
    }

    async fn make_github_request<T>(
        &self,
        url: &str,
        query_params: &[(&str, &str)],
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        let resp = self
            .client
            .get(url)
            .query(query_params)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send request: {}", e);
                e
            })?;
        match resp.status() {
            reqwest::StatusCode::OK => {
                let data: T = resp.json().await?;
                Ok(data)
            }
            status => {
                let error_body = resp.text().await?;
                Err(format!("GitHub API error: {} - {}", status, error_body).into())
            }
        }
    }
}
