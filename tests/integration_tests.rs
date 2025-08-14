use assert_fs::prelude::*;
use assert_fs::TempDir;
use zackstrap::ConfigGenerator;

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
    let editor_config = std::fs::read_to_string(temp_dir.child(".editorconfig").path()).unwrap();
    assert!(editor_config.contains("root = true"));
    assert!(editor_config.contains("charset = utf-8"));
    assert!(editor_config.contains("end_of_line = lf"));
    // Verify .prettierrc content
    let prettier_config = std::fs::read_to_string(temp_dir.child(".prettierrc").path()).unwrap();
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
    let ruby_version = std::fs::read_to_string(temp_dir.child(".ruby-version").path()).unwrap();
    assert_eq!(ruby_version.trim(), "3.3.0");

    let node_version = std::fs::read_to_string(temp_dir.child(".node-version").path()).unwrap();
    assert_eq!(node_version.trim(), "24");

    let package_json = std::fs::read_to_string(temp_dir.child("package.json").path()).unwrap();
    assert!(package_json.contains("prettier-plugin-ruby"));
    assert!(package_json.contains("github:prettier/plugin-ruby"));

    let rubocop_config = std::fs::read_to_string(temp_dir.child(".rubocop.yml").path()).unwrap();
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

    let editor_config = std::fs::read_to_string(temp_dir.child(".editorconfig").path()).unwrap();

    // Check for default section
    assert!(editor_config.contains("[*]"));
    assert!(editor_config.contains("indent_style = space"));
    assert!(editor_config.contains("indent_size = 2"));

    // Check for Ruby section
    assert!(editor_config.contains("[*.{rb,erb,ru,rake,gemspec}]"));

    // Check for JavaScript/TypeScript section
    assert!(editor_config.contains("[*.{yml,yaml,haml,jbuilder,jsx,html,sls,tf}]"));

    // makefiles
    assert!(editor_config.contains("[{*[Mm]akefile*,*.mak,*.mk,depend}]"));

    // enc/
    assert!(editor_config.contains("[enc/*]"));

    // reg  *.[ch]
    assert!(editor_config.contains("[reg*.[ch]]"));
}
