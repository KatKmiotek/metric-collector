use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MetricType {
    Workflow,
    PullRequest,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_workflow_metric_serialization() {
        let metric = ProjectMetric {
            project_name: "test-project".to_string(),
            metric_type: MetricType::Workflow,
            workflow_id: Some(123),
            workflow_name: Some("CI".to_string()),
            pull_request_id: None,
            result: Some("success".to_string()),
            duration: "5m".to_string(),
        };

        let json = serde_json::to_string(&metric).unwrap();
        let deserialized: ProjectMetric = serde_json::from_str(&json).unwrap();

        assert_eq!(metric.project_name, deserialized.project_name);
        assert_eq!(metric.metric_type, deserialized.metric_type);
        assert_eq!(metric.workflow_id, deserialized.workflow_id);
        assert_eq!(metric.workflow_name, deserialized.workflow_name);
        assert_eq!(metric.pull_request_id, deserialized.pull_request_id);
        assert_eq!(metric.result, deserialized.result);
        assert_eq!(metric.duration, deserialized.duration);
    }

    #[test]
    fn test_pull_request_metric_serialization() {
        let metric = ProjectMetric {
            project_name: "test-project".to_string(),
            metric_type: MetricType::PullRequest,
            workflow_id: None,
            workflow_name: None,
            pull_request_id: Some(456),
            result: Some("merged".to_string()),
            duration: "30m".to_string(),
        };

        let json = serde_json::to_string(&metric).unwrap();
        let deserialized: ProjectMetric = serde_json::from_str(&json).unwrap();

        assert_eq!(metric.project_name, deserialized.project_name);
        assert_eq!(metric.metric_type, deserialized.metric_type);
        assert_eq!(metric.workflow_id, deserialized.workflow_id);
        assert_eq!(metric.workflow_name, deserialized.workflow_name);
        assert_eq!(metric.pull_request_id, deserialized.pull_request_id);
        assert_eq!(metric.result, deserialized.result);
        assert_eq!(metric.duration, deserialized.duration);
    }

    #[test]
    fn test_optional_fields_omitted() {
        let metric = ProjectMetric {
            project_name: "test-project".to_string(),
            metric_type: MetricType::Workflow,
            workflow_id: None,
            workflow_name: None,
            pull_request_id: None,
            result: None,
            duration: "1h".to_string(),
        };

        let json = serde_json::to_string(&metric).unwrap();

        assert!(!json.contains("workflow_id"));
        assert!(!json.contains("workflow_name"));
        assert!(!json.contains("pull_request_id"));
        assert!(!json.contains("result"));
    }

    #[test]
    fn test_metric_type_serialization() {
        let workflow_type = MetricType::Workflow;
        let pr_type = MetricType::PullRequest;

        let workflow_json = serde_json::to_string(&workflow_type).unwrap();
        let pr_json = serde_json::to_string(&pr_type).unwrap();

        assert_eq!(workflow_json, "\"Workflow\"");
        assert_eq!(pr_json, "\"PullRequest\"");

        let deserialized_workflow: MetricType = serde_json::from_str(&workflow_json).unwrap();
        let deserialized_pr: MetricType = serde_json::from_str(&pr_json).unwrap();

        assert_eq!(workflow_type, deserialized_workflow);
        assert_eq!(pr_type, deserialized_pr);
    }
}
