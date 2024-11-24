use std::error::Error;

#[derive(Debug, Clone)]
pub struct GithubConfig {
    pub github_token: String,
    pub github_url: String,
    pub owner: String,
    pub repo: String,
}
impl GithubConfig {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(GithubConfig {
            github_token: std::env::var("GITHUB_TOKEN").expect("GitHub token is missing"),
            github_url: std::env::var("GITHUB_URL").expect("GitHub URL is missing"),
            owner: std::env::var("OWNER").expect("GitHub owner is missing"),
            repo: std::env::var("REPO").expect("GitHub repo is missing"),
        })
    }
}
