use crate::metric_models::ProjectMetric;
use std::{
    error::Error,
    fs::{self, File},
    path::Path,
};

pub struct SaveData;
impl SaveData {
    pub fn save_to_file(data: &Vec<ProjectMetric>) -> Result<(), Box<dyn Error>> {
        let dir_path = Path::new("output");
        let file_path = dir_path.join("metrics.txt");

        if !dir_path.exists() {
            fs::create_dir_all(dir_path)?;
        }
        let file = File::create(file_path)?;
        serde_json::to_writer_pretty(file, data)?;
        Ok(())
    }
}
