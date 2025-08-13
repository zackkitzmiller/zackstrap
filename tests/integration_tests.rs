use assert_fs::prelude::*;
use assert_fs::TempDir;
use zackstrap::{ConfigGenerator, ProjectType};

#[tokio::test]
async fn test_generate_basic_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test basic generation
    generator.generate_basic(false).await.unwrap();

    // Verify files were created
    temp_dir.child(".editorconfig").assert(predicates::path::exists());
    temp_dir.child(".prettierrc").assert(predicates::path::exists());

    // Verify .editorconfig content
    let editor_config = temp_dir.child(".editorconfig").read_to_string().unwrap();
    assert!(editor_config.contains("root = true"));
    assert!(editor_config.contains("charset = utf-8"));
    assert!(editor_config.contains("end_of_line = lf"));

    // Verify .prettierrc content
    let prettier_config = temp_dir.child(".prettierrc").read_to_string().unwrap();
    assert!(prettier_config.contains("\"semi\": true"));
    assert!(prettier_config.contains("\"single_quote\": true"));
}

#[tokio::test]
async fn test_generate_ruby_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Ruby generation
    generator.generate_ruby(false).await.unwrap();

    // Verify all files were created
    temp_dir.child(".editorconfig").assert(predicates::path::exists());
    temp_dir.child(".prettierrc").assert(predicates::path::exists());
    temp_dir.child(".ruby-version").assert(predicates::path::exists());
    temp_dir.child(".node-version").assert(predicates::path::exists());
    temp_dir.child(".rubocop.yml").assert(predicates::path::exists());
    temp_dir.child("package.json").assert(predicates::path::exists());

    // Verify Ruby-specific content
    let ruby_version = temp_dir.child(".ruby-version").read_to_string().unwrap();
    assert_eq!(ruby_version.trim(), "3.3.0");

    let node_version = temp_dir.child(".node-version").read_to_string().unwrap();
    assert_eq!(node_version.trim(), "24");

    let package_json = temp_dir.child("package.json").read_to_string().unwrap();
    assert!(package_json.contains("prettier-plugin-ruby"));
    assert!(package_json.contains("github:prettier/plugin-ruby"));

    let rubocop_config = temp_dir.child(".rubocop.yml").read_to_string().unwrap();
    assert!(rubocop_config.contains("TargetRubyVersion: 3.3"));
    assert!(rubocop_config.contains("Max: 120"));
}

#[tokio::test]
async fn test_force_overwrite() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Create initial config
    generator.generate_basic(false).await.unwrap();

    // Try to generate again without force - should fail
    let result = generator.generate_basic(false).await;
    assert!(result.is_err());

    // Generate with force - should succeed
    generator.generate_basic(true).await.unwrap();
}

#[tokio::test]
async fn test_editor_config_sections() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    generator.generate_basic(false).await.unwrap();

    let editor_config = temp_dir.child(".editorconfig").read_to_string().unwrap();

    // Check for default section
    assert!(editor_config.contains("[*]"));
    assert!(editor_config.contains("indent_style = space"));
    assert!(editor_config.contains("indent_size = 2"));

    // Check for Ruby section
    assert!(editor_config.contains("[*.rb]"));

    // Check for JavaScript/TypeScript section
    assert!(editor_config.contains("[*.{js,jsx,ts,tsx}]"));
}
