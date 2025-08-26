use crate::config::{EditorConfig, PrettierConfig};
use crate::error::ZackstrapError;
use super::common::FileGenerator;

impl super::ConfigGenerator {
    pub async fn generate_basic(&self, force: bool, fail_on_exists: bool) -> Result<(), ZackstrapError> {
        self.generate_basic_with_template(force, fail_on_exists, "default").await
    }

    pub async fn generate_basic_with_template(
        &self,
        force: bool,
        fail_on_exists: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        self.generate_editor_config(force, fail_on_exists).await?;
        self.generate_prettier_config_with_template(force, fail_on_exists, template).await?;
        self.generate_justfile(force, fail_on_exists).await?;
        Ok(())
    }

    pub async fn dry_run_basic_with_template(&self, template: &str) -> Result<(), ZackstrapError> {
        println!("  📝 Would generate .editorconfig");
        println!("  🎨 Would generate .prettierrc (template: {})", template);
        println!("  🔧 Would generate justfile");
        Ok(())
    }

    async fn generate_editor_config(&self, force: bool, fail_on_exists: bool) -> Result<(), ZackstrapError> {
        let config = EditorConfig::default();
        let content = config.to_string();
        self.write_file_if_not_exists(".editorconfig", &content, force, fail_on_exists).await
    }

    async fn generate_prettier_config_with_template(
        &self,
        force: bool,
        fail_on_exists: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let config = PrettierConfig::from_template(template);
        let content = config.to_string();
        self.write_file_if_not_exists(".prettierrc", &content, force, fail_on_exists).await
    }

    async fn generate_justfile(&self, force: bool, fail_on_exists: bool) -> Result<(), ZackstrapError> {
        let content = r#"# Basic project justfile
# Add your project-specific commands here

# Default target
default:
    @echo "Available commands:"
    @just --list

# Install dependencies
install:
    @echo "Installing dependencies..."

# Run tests
test:
    @echo "Running tests..."

# Build project
build:
    @echo "Building project..."

# Clean build artifacts
clean:
    @echo "Cleaning build artifacts..."
"#;
        self.write_file_if_not_exists("justfile", content, force, fail_on_exists).await
    }
}
