use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum MetricType {
    Workflow,
    PullRequest,
}

#[derive(Debug, Serialize)]
pub struct ProjectMetric {
    pub project_name: String,
    pub metric_type: MetricType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    pub duration: String,
}
