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

    /// Force overwrite existing files
    #[arg(short, long)]
    force: bool,

    /// Show what would be created without actually creating files
    #[arg(long)]
    dry_run: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate configuration files for a basic project
    Basic {
        /// Template to use (e.g., 'google', 'airbnb')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Generate configuration files for a Ruby project
    Ruby {
        /// Template to use (e.g., 'rails', 'sinatra', 'gem')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Generate configuration files for a Python project
    Python {
        /// Template to use (e.g., 'default', 'django', 'flask')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Generate configuration files for a Node.js project
    Node {
        /// Template to use (e.g., 'default', 'express', 'react')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Generate configuration files for a Go project
    Go {
        /// Template to use (e.g., 'default', 'web', 'cli')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Generate configuration files for a Rust project
    Rust {
        /// Template to use (e.g., 'default', 'web', 'cli')
        #[arg(short, long)]
        template: Option<String>,
    },
    /// Automatically detect project type and generate appropriate configs
    Auto,
    /// Interactive mode - guided configuration setup
    Interactive,
    /// List all available configuration files
    List,
}

#[tokio::main]
async fn main() -> Result<(), ZackstrapError> {
    let cli = Cli::parse();

    let target_dir = cli
        .target
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    if !target_dir.exists() {
        return Err(ZackstrapError::DirectoryNotFound(target_dir));
    }

    if !target_dir.is_dir() {
        return Err(ZackstrapError::NotADirectory(target_dir));
    }

    match cli.command {
        Commands::Basic { template } => {
            let template_name = template.as_deref().unwrap_or("default");
            if cli.dry_run {
                println!(
                    "{}",
                    format!(
                        "🚀 [DRY RUN] Would generate basic project configuration (template: {})...",
                        template_name
                    )
                    .blue()
                );
                let generator = ConfigGenerator::new(target_dir);
                generator.dry_run_basic_with_template(template_name).await?;
            } else {
                println!(
                    "{}",
                    format!(
                        "🚀 Generating basic project configuration (template: {})...",
                        template_name
                    )
                    .green()
                );
                let generator = ConfigGenerator::new(target_dir);
                generator
                    .generate_basic_with_template(cli.force, template_name)
                    .await?;
                println!(
                    "{}",
                    "✅ Basic configuration files generated successfully!".green()
                );
            }
        }
        Commands::Ruby { template } => {
            let template_name = template.as_deref().unwrap_or("default");
            if cli.dry_run {
                println!(
                    "{}",
                    format!(
                        "💎 [DRY RUN] Would generate Ruby project configuration (template: {})...",
                        template_name
                    )
                    .blue()
                );
                let generator = ConfigGenerator::new(target_dir);
                generator.dry_run_ruby_with_template(template_name).await?;
            } else {
                println!(
                    "{}",
                    format!(
                        "💎 Generating Ruby project configuration (template: {})...",
                        template_name
                    )
                    .green()
                );
                let generator = ConfigGenerator::new(target_dir);
                generator
                    .generate_ruby_with_template(cli.force, template_name)
                    .await?;
                println!(
                    "{}",
                    "✅ Ruby configuration files generated successfully!".green()
                );
            }
        }
        Commands::Python { template } => {
            let template_name = template.as_deref().unwrap_or("default");
            if cli.dry_run {
                println!("{}", format!("🐍 [DRY RUN] Would generate Python project configuration (template: {})...", template_name).blue());
                let generator = ConfigGenerator::new(target_dir);
                generator
                    .dry_run_python_with_template(template_name)
                    .await?;
            } else {
                println!(
                    "{}",
                    format!(
                        "🐍 Generating Python project configuration (template: {})...",
                        template_name
                    )
                    .green()
                );
                let generator = ConfigGenerator::new(target_dir);
                generator
                    .generate_python_with_template(cli.force, template_name)
                    .await?;
                println!(
                    "{}",
                    "✅ Python configuration files generated successfully!".green()
                );
            }
        }
        Commands::Node { template } => {
            let template_name = template.as_deref().unwrap_or("default");
            if cli.dry_run {
                println!("{}", format!("🟢 [DRY RUN] Would generate Node.js project configuration (template: {})...", template_name).blue());
                let generator = ConfigGenerator::new(target_dir);
                generator.dry_run_node_with_template(template_name).await?;
            } else {
                println!(
                    "{}",
                    format!(
                        "🟢 Generating Node.js project configuration (template: {})...",
                        template_name
                    )
                    .green()
                );
                let generator = ConfigGenerator::new(target_dir);
                generator
                    .generate_node_with_template(cli.force, template_name)
                    .await?;
                println!(
                    "{}",
                    "✅ Node.js configuration files generated successfully!".green()
                );
            }
        }
        Commands::Go { template } => {
            let template_name = template.as_deref().unwrap_or("default");
            if cli.dry_run {
                println!(
                    "{}",
                    format!(
                        "🦀 [DRY RUN] Would generate Go project configuration (template: {})...",
                        template_name
                    )
                    .blue()
                );
                let generator = ConfigGenerator::new(target_dir);
                generator.dry_run_go_with_template(template_name).await?;
            } else {
                println!(
                    "{}",
                    format!(
                        "🦀 Generating Go project configuration (template: {})...",
                        template_name
                    )
                    .green()
                );
                let generator = ConfigGenerator::new(target_dir);
                generator
                    .generate_go_with_template(cli.force, template_name)
                    .await?;
                println!(
                    "{}",
                    "✅ Go configuration files generated successfully!".green()
                );
            }
        }
        Commands::Rust { template } => {
            let template_name = template.as_deref().unwrap_or("default");
            if cli.dry_run {
                println!(
                    "{}",
                    format!(
                        "🦀 [DRY RUN] Would generate Rust project configuration (template: {})...",
                        template_name
                    )
                    .blue()
                );
                let generator = ConfigGenerator::new(target_dir);
                generator.dry_run_rust_with_template(template_name).await?;
            } else {
                println!(
                    "{}",
                    format!(
                        "🦀 Generating Rust project configuration (template: {})...",
                        template_name
                    )
                    .green()
                );
                let generator = ConfigGenerator::new(target_dir);
                generator
                    .generate_rust_with_template(cli.force, template_name)
                    .await?;
                println!(
                    "{}",
                    "✅ Rust configuration files generated successfully!".green()
                );
            }
        }
        Commands::Auto => {
            if cli.dry_run {
                println!("{}", "🔍 [DRY RUN] Auto-detecting project type...".blue());
                let generator = ConfigGenerator::new(target_dir);
                let project_type = generator.detect_project_type().await?;
                match project_type {
                    ProjectType::Ruby => {
                        println!(
                            "{}",
                            "💎 [DRY RUN] Would generate Ruby project configuration...".blue()
                        );
                        generator.dry_run_ruby_with_template("default").await?;
                    }
                    ProjectType::Python => {
                        println!(
                            "{}",
                            "🐍 [DRY RUN] Would generate Python project configuration...".blue()
                        );
                        generator.dry_run_python_with_template("default").await?;
                    }
                    ProjectType::Node => {
                        println!(
                            "{}",
                            "🟢 [DRY RUN] Would generate Node.js project configuration...".blue()
                        );
                        generator.dry_run_node_with_template("default").await?;
                    }
                    ProjectType::Go => {
                        println!(
                            "{}",
                            "🦀 [DRY RUN] Would generate Go project configuration...".blue()
                        );
                        generator.dry_run_go_with_template("default").await?;
                    }
                    ProjectType::Rust => {
                        println!(
                            "{}",
                            "🦀 [DRY RUN] Would generate Rust project configuration...".blue()
                        );
                        generator.dry_run_rust_with_template("default").await?;
                    }
                    ProjectType::Basic => {
                        println!(
                            "{}",
                            "📁 [DRY RUN] Would generate basic project configuration...".blue()
                        );
                        generator.dry_run_basic_with_template("default").await?;
                    }
                }
            } else {
                println!("{}", "🔍 Auto-detecting project type...".blue());
                let generator = ConfigGenerator::new(target_dir);
                let project_type = generator.detect_project_type().await?;
                match project_type {
                    ProjectType::Ruby => {
                        println!(
                            "{}",
                            "💎 Detected Ruby project, generating configuration...".green()
                        );
                        generator.generate_ruby(cli.force).await?;
                        println!(
                            "{}",
                            "✅ Ruby configuration files generated successfully!".green()
                        );
                    }
                    ProjectType::Python => {
                        println!(
                            "{}",
                            "🐍 Detected Python project, generating configuration...".green()
                        );
                        generator
                            .generate_python_with_template(cli.force, "default")
                            .await?;
                        println!(
                            "{}",
                            "✅ Python configuration files generated successfully!".green()
                        );
                    }
                    ProjectType::Node => {
                        println!(
                            "{}",
                            "🟢 Detected Node.js project, generating configuration...".green()
                        );
                        generator
                            .generate_node_with_template(cli.force, "default")
                            .await?;
                        println!(
                            "{}",
                            "✅ Node.js configuration files generated successfully!".green()
                        );
                    }
                    ProjectType::Go => {
                        println!(
                            "{}",
                            "🦀 Detected Go project, generating configuration...".green()
                        );
                        generator
                            .generate_go_with_template(cli.force, "default")
                            .await?;
                        println!(
                            "{}",
                            "✅ Go configuration files generated successfully!".green()
                        );
                    }
                    ProjectType::Rust => {
                        println!(
                            "{}",
                            "🦀 Detected Rust project, generating configuration...".green()
                        );
                        generator
                            .generate_rust_with_template(cli.force, "default")
                            .await?;
                        println!(
                            "{}",
                            "✅ Rust configuration files generated successfully!".green()
                        );
                    }
                    ProjectType::Basic => {
                        println!(
                            "{}",
                            "📁 Detected basic project, generating configuration...".green()
                        );
                        generator.generate_basic(cli.force).await?;
                        println!(
                            "{}",
                            "✅ Basic configuration files generated successfully!".green()
                        );
                    }
                }
            }
        }
        Commands::Interactive => {
            if cli.dry_run {
                println!(
                    "{}",
                    "🎯 [DRY RUN] Interactive configuration setup...".blue()
                );
                println!("  Note: Dry run mode is not fully supported in interactive mode");
                let generator = ConfigGenerator::new(target_dir);
                generator.interactive_setup().await?;
            } else {
                println!("{}", "🎯 Interactive configuration setup...".blue());
                let generator = ConfigGenerator::new(target_dir);
                generator.interactive_setup().await?;
            }
        }
        Commands::List => {
            println!("{}", "📋 Available configuration files:".blue());
            println!("  • .editorconfig");
            println!("  • .prettierrc");
            println!("  • .ruby-version (Ruby projects)");
            println!("  • .node-version (Ruby projects)");
            println!("  • .rubocop.yml (Ruby projects)");
            println!("  • package.json (Ruby projects)");
            println!("  • .python-version (Python projects)");
            println!("  • pyproject.toml (Python projects)");
            println!("  • .flake8 (Python projects)");
            println!("  • requirements-dev.txt (Python projects)");
            println!("  • .nvmrc (Node.js projects)");
            println!("  • .eslintrc.js (Node.js projects)");
            println!("  • go.mod (Go projects)");
            println!("  • .golangci.yml (Go projects)");
            println!("  • rustfmt.toml (Rust projects)");
            println!("  • .clippy.toml (Rust projects)");
            println!("  • .cargo/config.toml (Rust projects)");
            println!("  • justfile (all projects)");
            println!();
            println!("📋 Available templates:");
            println!("  • Basic: default, google, airbnb");
            println!("  • Ruby: default, rails, sinatra, gem");
            println!("  • Python: default, django, flask");
            println!("  • Node.js: default, express, react");
            println!("  • Go: default, web, cli");
            println!("  • Rust: default, web, cli");
            println!();
            println!("🚀 Available commands:");
            println!("  • basic - Generate basic project configs");
            println!("  • ruby - Generate Ruby project configs");
            println!("  • python - Generate Python project configs");
            println!("  • node - Generate Node.js project configs");
            println!("  • go - Generate Go project configs");
            println!("  • rust - Generate Rust project configs");
            println!("  • auto - Auto-detect project type");
            println!("  • interactive - Guided setup");
            println!("  • list - Show this help");
            println!();
            println!("⚙️  Global options:");
            println!("  • --force - Overwrite existing files");
            println!("  • --dry-run - Show what would be created");
            println!("  • --target DIR - Specify target directory");
        }
    }

    Ok(())
}
