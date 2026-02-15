use crate::config::Config;
use crate::github::{GitHubClient, GitHubUser};
use tera::{Context, Tera};

pub struct ProfileGenerator {
    config: Config,
    client: GitHubClient,
}

impl ProfileGenerator {
    pub fn new(config: Config) -> Self {
        let client = GitHubClient::new(config.github_username.clone());
        Self { config, client }
    }

    pub async fn generate(&self) -> anyhow::Result<String> {
        // Fetch data from GitHub
        let user = self.client.get_user().await?;
        let repos = self.client.get_repositories().await?;
        let total_stars = self.client.get_total_stars().await?;
        let languages = self.client.get_languages().await?;

        // Create template context
        let mut context = Context::new();
        context.insert("user", &user);
        context.insert("config", &self.config);
        context.insert("total_stars", &total_stars);
        context.insert("languages", &languages);
        context.insert("repos", &repos);

        // Render template
        let template = include_str!("../templates/profile.md");
        let mut tera = Tera::default();
        tera.add_raw_template("profile", template)?;
        
        let rendered = tera.render("profile", &context)?;
        Ok(rendered)
    }

    pub fn generate_stats_badges(&self, user: &GitHubUser, total_stars: u32) -> String {
        let username = &user.login;
        format!(
            "![GitHub followers](https://img.shields.io/github/followers/{}?style=social) \
             ![GitHub stars](https://img.shields.io/badge/stars-{}-yellow) \
             ![GitHub repos](https://img.shields.io/badge/repos-{}-blue)",
            username, total_stars, user.public_repos
        )
    }

    pub fn generate_language_badges(&self, languages: &[String]) -> String {
        languages
            .iter()
            .map(|lang| {
                format!(
                    "![{}](https://img.shields.io/badge/-{}-informational?style=flat&logo={})",
                    lang,
                    lang,
                    lang.to_lowercase()
                )
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}
