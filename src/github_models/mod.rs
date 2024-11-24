use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct WorkflowRunsResponse {
    pub total_count: u32,
    pub workflow_runs: Vec<WorkflowRun>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct WorkflowRun {
    pub id: u64,
    pub updated_at: DateTime<Utc>,
    pub run_started_at: Option<DateTime<Utc>>,
}

#[derive(serde::Deserialize)]
pub struct PullRequest {
    pub id: u64,
    pub created_at: DateTime<Utc>,
    pub merged_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub enum Conclusion {
    Success,
    Failure,
}
impl Conclusion {
    pub fn as_str(&self) -> &'static str {
        match self {
            Conclusion::Success => "success",
            Conclusion::Failure => "failure",
        }
    }
}

#[derive(Debug)]
pub enum RunName {
    PullRequest,
    Release,
}

impl RunName {
    pub fn as_str(&self) -> &'static str {
        match self {
            RunName::PullRequest => "Pull Request",
            RunName::Release => "Release",
        }
    }
}
