use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub github_username: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub show_stats: bool,
    pub show_languages: bool,
    pub show_recent_repos: bool,
    pub custom_sections: Vec<CustomSection>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CustomSection {
    pub title: String,
    pub content: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn default_config(username: String) -> Self {
        Self {
            github_username: username,
            title: None,
            subtitle: None,
            show_stats: true,
            show_languages: true,
            show_recent_repos: true,
            custom_sections: vec![],
        }
    }
}
