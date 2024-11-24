#[derive(Debug)]
pub struct Metric {
    pub project_name: String,
    pub workflow_id: u64,
    pub workflow_name: String,
    pub result: String,
    pub duration: String
}

pub struct Metrics {
    pub source: String,
    pub metrics: Vec<Metric>
}
