use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

mod config;
mod error;
mod generators;

use error::ZackstrapError;
use generators::{ConfigGenerator, ProjectType};

#[derive(Parser)]
#[command(
    name = "zackstrap",
    about = "Bootstrap project configuration files",
    version,
    long_about = "A CLI tool to quickly generate common project configuration files like .editorconfig, .prettierrc, and more."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Target directory (defaults to current directory)
    #[arg(short, long, value_name = "DIR")]
    target: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate configuration files for a basic project
    Basic {
        /// Force overwrite existing files
        #[arg(short, long)]
        force: bool,
    },
    /// Generate configuration files for a Ruby project
    Ruby {
        /// Force overwrite existing files
        #[arg(short, long)]
        force: bool,
    },
    /// List all available configuration files
    List,
}

#[tokio::main]
async fn main() -> Result<(), ZackstrapError> {
    let cli = Cli::parse();

    let target_dir = cli.target.unwrap_or_else(|| {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    });

    if !target_dir.exists() {
        return Err(ZackstrapError::DirectoryNotFound(target_dir));
    }

    if !target_dir.is_dir() {
        return Err(ZackstrapError::NotADirectory(target_dir));
    }

    match cli.command {
        Commands::Basic { force } => {
            println!("{}", "🚀 Generating basic project configuration...".green());
            let generator = ConfigGenerator::new(target_dir);
            generator.generate_basic(force).await?;
            println!("{}", "✅ Basic configuration files generated successfully!".green());
        }
        Commands::Ruby { force } => {
            println!("{}", "💎 Generating Ruby project configuration...".green());
            let generator = ConfigGenerator::new(target_dir);
            generator.generate_ruby(force).await?;
            println!("{}", "✅ Ruby configuration files generated successfully!".green());
        }
        Commands::List => {
            println!("{}", "📋 Available configuration files:".blue());
            println!("  • .editorconfig");
            println!("  • .prettierrc");
            println!("  • .ruby-version (Ruby projects)");
            println!("  • .node-version (Ruby projects)");
            println!("  • .rubocop.yml (Ruby projects)");
            println!("  • package.json (Ruby projects)");
        }
    }

    Ok(())
}
