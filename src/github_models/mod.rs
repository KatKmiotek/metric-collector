use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WorkflowRunsResponse {
    pub total_count: u32,
    // pub workflow_runs: Vec<WorkflowRun>,
}

#[derive(Debug, Deserialize)]
pub struct WorkflowRun {
    // pub run_started_at: String
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
    Release
}

impl RunName {
    pub fn as_str(&self) -> &'static str {
        match self {
            RunName::PullRequest => "Pull Request",
            RunName::Release => "Release"
        }
    }
}
