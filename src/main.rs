mod github;
mod config;
mod generator;

use clap::Parser;
use config::Config;
use generator::ProfileGenerator;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "github-profile-rust")]
#[command(about = "Generate a dynamic GitHub profile README", long_about = None)]
struct Args {
    /// GitHub username
    #[arg(short, long)]
    username: Option<String>,

    /// Path to config file
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,

    /// Output file path
    #[arg(short, long, default_value = "README.md")]
    output: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Load or create config
    let config = if args.config.exists() {
        println!("📖 Loading config from {:?}", args.config);
        Config::from_file(&args.config)?
    } else if let Some(username) = args.username {
        println!("⚙️  Using default config for user: {}", username);
        Config::default_config(username)
    } else {
        anyhow::bail!("Please provide either a config file or a username via --username");
    };

    println!("🚀 Generating profile for: {}", config.github_username);

    // Generate profile
    let generator = ProfileGenerator::new(config);
    let readme = generator.generate().await?;

    // Write to file
    fs::write(&args.output, readme)?;
    println!("✅ Profile generated successfully: {:?}", args.output);

    Ok(())
}
