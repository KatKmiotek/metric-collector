use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WorkflowMetric {
    pub project_name: String,
    pub workflow_id: u64,
    pub workflow_name: String,
    pub result: String,
    pub duration: String,
    pub event: String,
}
#[derive(Debug, Serialize)]
pub struct PullRequestMetric {
    pub project_name: String,
    pub pull_request_id: u64,
    pub duration: String,
    pub event: String,
}

#[derive(Debug, Serialize)]
pub enum GitHubMetric {
    Workflow(WorkflowMetric),
    PullRequest(PullRequestMetric),
}
