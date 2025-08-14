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
    temp_dir
        .child(".editorconfig")
        .assert(predicates::path::exists());
    temp_dir
        .child(".prettierrc")
        .assert(predicates::path::exists());
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
    temp_dir
        .child(".editorconfig")
        .assert(predicates::path::exists());
    temp_dir
        .child(".prettierrc")
        .assert(predicates::path::exists());
    temp_dir
        .child(".ruby-version")
        .assert(predicates::path::exists());
    temp_dir
        .child(".node-version")
        .assert(predicates::path::exists());
    temp_dir
        .child(".rubocop.yml")
        .assert(predicates::path::exists());
    temp_dir
        .child("package.json")
        .assert(predicates::path::exists());
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
async fn test_generate_python_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Python generation
    generator
        .generate_python_with_template(false, "default")
        .await
        .unwrap();

    // Verify all files were created
    temp_dir
        .child(".editorconfig")
        .assert(predicates::path::exists());
    temp_dir
        .child(".prettierrc")
        .assert(predicates::path::exists());
    temp_dir
        .child(".python-version")
        .assert(predicates::path::exists());
    temp_dir
        .child("pyproject.toml")
        .assert(predicates::path::exists());
    temp_dir.child(".flake8").assert(predicates::path::exists());
    temp_dir
        .child("requirements-dev.txt")
        .assert(predicates::path::exists());

    // Verify Python-specific content
    let python_version = std::fs::read_to_string(temp_dir.child(".python-version").path()).unwrap();
    assert_eq!(python_version.trim(), "3.12");

    let pyproject_toml = std::fs::read_to_string(temp_dir.child("pyproject.toml").path()).unwrap();
    assert!(pyproject_toml.contains("line-length = 88"));
    assert!(pyproject_toml.contains("target-version = ['py312']"));
    assert!(pyproject_toml.contains("strict = true"));

    let flake8_config = std::fs::read_to_string(temp_dir.child(".flake8").path()).unwrap();
    assert!(flake8_config.contains("max-line-length = 88"));
    assert!(flake8_config.contains("extend-ignore = E203, W503"));

    let requirements_dev =
        std::fs::read_to_string(temp_dir.child("requirements-dev.txt").path()).unwrap();
    assert!(requirements_dev.contains("black=="));
    assert!(requirements_dev.contains("flake8=="));
    assert!(requirements_dev.contains("mypy=="));
    assert!(requirements_dev.contains("pytest=="));
}

#[tokio::test]
async fn test_generate_python_config_with_templates() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Django template
    generator
        .generate_python_with_template(false, "django")
        .await
        .unwrap();
    let pyproject_toml = std::fs::read_to_string(temp_dir.child("pyproject.toml").path()).unwrap();
    assert!(pyproject_toml.contains("django_settings_module"));

    // Clean and test Flask template
    std::fs::remove_file(temp_dir.child("pyproject.toml").path()).unwrap();
    std::fs::remove_file(temp_dir.child("justfile").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".editorconfig").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".prettierrc").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".python-version").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".flake8").path()).unwrap();
    std::fs::remove_file(temp_dir.child("requirements-dev.txt").path()).unwrap();
    generator
        .generate_python_with_template(false, "flask")
        .await
        .unwrap();
    let pyproject_toml = std::fs::read_to_string(temp_dir.child("pyproject.toml").path()).unwrap();
    assert!(pyproject_toml.contains("app_name = \"app\""));
}

#[tokio::test]
async fn test_generate_node_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Node.js generation
    generator
        .generate_node_with_template(false, "default")
        .await
        .unwrap();

    // Verify all files were created
    temp_dir
        .child(".editorconfig")
        .assert(predicates::path::exists());
    temp_dir
        .child(".prettierrc")
        .assert(predicates::path::exists());
    temp_dir.child(".nvmrc").assert(predicates::path::exists());
    temp_dir
        .child(".eslintrc.js")
        .assert(predicates::path::exists());
    temp_dir
        .child("package.json")
        .assert(predicates::path::exists());

    // Verify Node.js-specific content
    let nvmrc = std::fs::read_to_string(temp_dir.child(".nvmrc").path()).unwrap();
    assert_eq!(nvmrc.trim(), "20");

    let eslint_config = std::fs::read_to_string(temp_dir.child(".eslintrc.js").path()).unwrap();
    assert!(eslint_config.contains("es2022"));
    assert!(eslint_config.contains("eslint:recommended"));

    let package_json = std::fs::read_to_string(temp_dir.child("package.json").path()).unwrap();
    assert!(package_json.contains("eslint"));
    assert!(package_json.contains("prettier"));
}

#[tokio::test]
async fn test_generate_node_config_with_templates() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Express template
    generator
        .generate_node_with_template(false, "express")
        .await
        .unwrap();
    let eslint_config = std::fs::read_to_string(temp_dir.child(".eslintrc.js").path()).unwrap();
    assert!(eslint_config.contains("no-console"));

    // Clean and test React template
    std::fs::remove_file(temp_dir.child(".eslintrc.js").path()).unwrap();
    std::fs::remove_file(temp_dir.child("package.json").path()).unwrap();
    std::fs::remove_file(temp_dir.child("justfile").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".editorconfig").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".prettierrc").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".nvmrc").path()).unwrap();
    generator
        .generate_node_with_template(false, "react")
        .await
        .unwrap();
    let eslint_config = std::fs::read_to_string(temp_dir.child(".eslintrc.js").path()).unwrap();
    assert!(eslint_config.contains("plugin:react/recommended"));
    assert!(eslint_config.contains("react/prop-types"));
}

#[tokio::test]
async fn test_generate_go_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Go generation
    generator
        .generate_go_with_template(false, "default")
        .await
        .unwrap();

    // Verify all files were created
    temp_dir
        .child(".editorconfig")
        .assert(predicates::path::exists());
    temp_dir
        .child(".prettierrc")
        .assert(predicates::path::exists());
    temp_dir.child("go.mod").assert(predicates::path::exists());
    temp_dir
        .child(".golangci.yml")
        .assert(predicates::path::exists());

    // Verify Go-specific content
    let go_mod = std::fs::read_to_string(temp_dir.child("go.mod").path()).unwrap();
    assert!(go_mod.contains("module myproject"));
    assert!(go_mod.contains("go 1.21"));

    let golangci_config = std::fs::read_to_string(temp_dir.child(".golangci.yml").path()).unwrap();
    assert!(golangci_config.contains("gofmt"));
    assert!(golangci_config.contains("golint"));
    assert!(golangci_config.contains("govet"));
    assert!(golangci_config.contains("errcheck"));
}

#[tokio::test]
async fn test_generate_go_config_with_templates() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test web template
    generator
        .generate_go_with_template(false, "web")
        .await
        .unwrap();
    let justfile = std::fs::read_to_string(temp_dir.child("justfile").path()).unwrap();
    assert!(justfile.contains("go-web"));

    // Clean and test CLI template
    std::fs::remove_file(temp_dir.child("justfile").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".editorconfig").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".prettierrc").path()).unwrap();
    std::fs::remove_file(temp_dir.child("go.mod").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".golangci.yml").path()).unwrap();
    // Note: .gitignore is updated, not created, so we don't remove it
    generator
        .generate_go_with_template(true, "cli")
        .await
        .unwrap();
    let justfile = std::fs::read_to_string(temp_dir.child("justfile").path()).unwrap();
    assert!(justfile.contains("go-cli"));
}

#[tokio::test]
async fn test_generate_rust_config() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Rust generation
    generator
        .generate_rust_with_template(false, "default")
        .await
        .unwrap();

    // Verify all files were created
    temp_dir
        .child(".editorconfig")
        .assert(predicates::path::exists());
    temp_dir
        .child(".prettierrc")
        .assert(predicates::path::exists());
    temp_dir
        .child("rustfmt.toml")
        .assert(predicates::path::exists());
    temp_dir
        .child(".clippy.toml")
        .assert(predicates::path::exists());
    temp_dir
        .child(".cargo/config.toml")
        .assert(predicates::path::exists());

    // Verify Rust-specific content
    let rustfmt_config = std::fs::read_to_string(temp_dir.child("rustfmt.toml").path()).unwrap();
    assert!(rustfmt_config.contains("edition = \"2021\""));
    assert!(rustfmt_config.contains("max_width = 100"));

    let cargo_config =
        std::fs::read_to_string(temp_dir.child(".cargo/config.toml").path()).unwrap();
    assert!(cargo_config.contains("target-cpu=native"));
}

#[tokio::test]
async fn test_generate_rust_config_with_templates() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test web template
    generator
        .generate_rust_with_template(false, "web")
        .await
        .unwrap();
    let justfile = std::fs::read_to_string(temp_dir.child("justfile").path()).unwrap();
    assert!(justfile.contains("cargo-web"));

    // Clean and test CLI template
    std::fs::remove_file(temp_dir.child("justfile").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".editorconfig").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".prettierrc").path()).unwrap();
    std::fs::remove_file(temp_dir.child("rustfmt.toml").path()).unwrap();
    std::fs::remove_file(temp_dir.child(".clippy.toml").path()).unwrap();
    std::fs::remove_dir_all(temp_dir.child(".cargo").path()).unwrap();
    generator
        .generate_rust_with_template(false, "cli")
        .await
        .unwrap();
    let justfile = std::fs::read_to_string(temp_dir.child("justfile").path()).unwrap();
    assert!(justfile.contains("cargo-cli"));
}

#[tokio::test]
async fn test_project_type_detection() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Python detection
    std::fs::write(temp_dir.child("main.py").path(), "print('Hello')").unwrap();
    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, zackstrap::ProjectType::Python));

    // Test Node.js detection
    std::fs::remove_file(temp_dir.child("main.py").path()).unwrap();
    std::fs::write(temp_dir.child("package.json").path(), "{}").unwrap();
    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, zackstrap::ProjectType::Node));

    // Test Go detection
    std::fs::remove_file(temp_dir.child("package.json").path()).unwrap();
    std::fs::write(temp_dir.child("main.go").path(), "package main").unwrap();
    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, zackstrap::ProjectType::Go));

    // Test Rust detection
    std::fs::remove_file(temp_dir.child("main.go").path()).unwrap();
    std::fs::write(temp_dir.child("main.rs").path(), "fn main() {}").unwrap();
    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, zackstrap::ProjectType::Rust));

    // Test Ruby detection
    std::fs::remove_file(temp_dir.child("main.rs").path()).unwrap();
    std::fs::write(
        temp_dir.child("Gemfile").path(),
        "source 'https://rubygems.org'",
    )
    .unwrap();
    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, zackstrap::ProjectType::Ruby));

    // Test basic detection (default)
    std::fs::remove_file(temp_dir.child("Gemfile").path()).unwrap();
    let project_type = generator.detect_project_type().await.unwrap();
    assert!(matches!(project_type, zackstrap::ProjectType::Basic));
}

#[tokio::test]
async fn test_dry_run_modes() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Python dry run
    generator
        .dry_run_python_with_template("default")
        .await
        .unwrap();

    // Test Node.js dry run
    generator
        .dry_run_node_with_template("default")
        .await
        .unwrap();

    // Test Go dry run
    generator.dry_run_go_with_template("default").await.unwrap();

    // Test Rust dry run
    generator
        .dry_run_rust_with_template("default")
        .await
        .unwrap();

    // Verify no files were actually created
    assert!(!temp_dir.child(".python-version").exists());
    assert!(!temp_dir.child(".nvmrc").exists());
    assert!(!temp_dir.child("go.mod").exists());
    assert!(!temp_dir.child("rustfmt.toml").exists());
}

#[tokio::test]
async fn test_force_overwrite_for_new_languages() {
    let temp_dir = TempDir::new().unwrap();
    let generator = ConfigGenerator::new(temp_dir.path().to_path_buf());

    // Test Python force overwrite
    generator
        .generate_python_with_template(false, "default")
        .await
        .unwrap();
    let result = generator
        .generate_python_with_template(false, "default")
        .await;
    assert!(result.is_err());
    generator
        .generate_python_with_template(true, "default")
        .await
        .unwrap();

    // Test Node.js force overwrite
    let _ = std::fs::remove_file(temp_dir.child("package.json").path());
    let _ = std::fs::remove_file(temp_dir.child(".nvmrc").path());
    let _ = std::fs::remove_file(temp_dir.child(".eslintrc.js").path());
    let _ = std::fs::remove_file(temp_dir.child("justfile").path());
    let _ = std::fs::remove_file(temp_dir.child(".editorconfig").path());
    let _ = std::fs::remove_file(temp_dir.child(".prettierrc").path());
    generator
        .generate_node_with_template(false, "default")
        .await
        .unwrap();
    let result = generator
        .generate_node_with_template(false, "default")
        .await;
    assert!(result.is_err());
    generator
        .generate_node_with_template(true, "default")
        .await
        .unwrap();
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
