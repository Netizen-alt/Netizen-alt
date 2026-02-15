use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GitHubUser {
    pub login: String,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub public_repos: u32,
    pub followers: u32,
    pub following: u32,
    pub avatar_url: String,
    pub html_url: String,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub twitter_username: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Repository {
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub language: Option<String>,
    pub updated_at: String,
}

pub struct GitHubClient {
    client: reqwest::Client,
    username: String,
}

impl GitHubClient {
    pub fn new(username: String) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("github-profile-rust")
            .build()
            .unwrap();
        
        Self { client, username }
    }

    pub async fn get_user(&self) -> anyhow::Result<GitHubUser> {
        let url = format!("https://api.github.com/users/{}", self.username);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch user data: {}", response.status());
        }
        
        let user: GitHubUser = response.json().await?;
        Ok(user)
    }

    pub async fn get_repositories(&self) -> anyhow::Result<Vec<Repository>> {
        let url = format!(
            "https://api.github.com/users/{}/repos?sort=updated&per_page=10",
            self.username
        );
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch repositories: {}", response.status());
        }
        
        let repos: Vec<Repository> = response.json().await?;
        Ok(repos)
    }

    pub async fn get_total_stars(&self) -> anyhow::Result<u32> {
        let repos = self.get_repositories().await?;
        let total_stars: u32 = repos.iter().map(|r| r.stargazers_count).sum();
        Ok(total_stars)
    }

    pub async fn get_languages(&self) -> anyhow::Result<Vec<String>> {
        let repos = self.get_repositories().await?;
        let mut languages: Vec<String> = repos
            .iter()
            .filter_map(|r| r.language.clone())
            .collect();
        
        languages.sort();
        languages.dedup();
        Ok(languages)
    }
}
