use crate::config::{EditorConfig, PackageJson, PrettierConfig};
use crate::error::ZackstrapError;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectType {
    Basic,
    Ruby,
    Python,
    Node,
    Go,
    Rust,
}

pub struct ConfigGenerator {
    target_dir: PathBuf,
}

impl ConfigGenerator {
    pub fn new(target_dir: PathBuf) -> Self {
        Self { target_dir }
    }

    pub async fn generate_basic(&self, force: bool) -> Result<(), ZackstrapError> {
        self.generate_basic_with_template(force, "default").await
    }

    pub async fn generate_basic_with_template(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        self.generate_editor_config(force).await?;
        self.generate_prettier_config_with_template(force, template)
            .await?;
        self.generate_justfile(force).await?;
        Ok(())
    }

    pub async fn dry_run_basic_with_template(&self, template: &str) -> Result<(), ZackstrapError> {
        println!("  📝 Would generate .editorconfig");
        println!("  🎨 Would generate .prettierrc (template: {})", template);
        println!("  🔧 Would generate justfile");
        Ok(())
    }

    pub async fn generate_ruby(&self, force: bool) -> Result<(), ZackstrapError> {
        self.generate_ruby_with_template(force, "default").await
    }

    pub async fn generate_ruby_with_template(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        // Generate basic configs first (includes justfile)
        self.generate_basic_with_template(force, template).await?;

        // Generate Ruby-specific configs
        self.generate_ruby_version(force).await?;
        self.generate_node_version(force).await?;
        self.generate_rubocop_config_with_template(force, template)
            .await?;
        self.generate_package_json_with_template(force, template)
            .await?;

        // Overwrite the basic justfile with Ruby-specific one
        self.generate_ruby_justfile(force, template).await?;

        Ok(())
    }

    pub async fn dry_run_ruby_with_template(&self, template: &str) -> Result<(), ZackstrapError> {
        println!("  📝 Would generate .editorconfig");
        println!("  🎨 Would generate .prettierrc (template: {})", template);
        println!("  🔧 Would generate justfile");
        println!("  💎 Would generate .ruby-version (3.3.0)");
        println!("  🟢 Would generate .node-version (24)");
        println!("  🔍 Would generate .rubocop.yml (template: {})", template);
        println!("  📦 Would generate package.json (template: {})", template);
        println!("  🔧 Would generate Ruby justfile (template: {})", template);
        Ok(())
    }

    // Python project generation
    pub async fn generate_python_with_template(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        // Generate basic configs first
        self.generate_basic_with_template(force, template).await?;

        // Generate Python-specific configs
        self.generate_python_version(force).await?;
        self.generate_pyproject_toml(force, template).await?;
        self.generate_flake8_config(force).await?;
        self.generate_requirements_dev(force).await?;

        // Overwrite the basic justfile with Python-specific one
        self.generate_python_justfile(force, template).await?;

        Ok(())
    }

    pub async fn dry_run_python_with_template(&self, template: &str) -> Result<(), ZackstrapError> {
        println!("  📝 Would generate .editorconfig");
        println!("  🎨 Would generate .prettierrc (template: {})", template);
        println!("  🔧 Would generate justfile");
        println!("  🐍 Would generate .python-version (3.12)");
        println!(
            "  📦 Would generate pyproject.toml (template: {})",
            template
        );
        println!("  🔍 Would generate .flake8");
        println!("  📋 Would generate requirements-dev.txt");
        println!(
            "  🔧 Would generate Python justfile (template: {})",
            template
        );
        Ok(())
    }

    // Node.js project generation
    pub async fn generate_node_with_template(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        // Generate basic configs first
        self.generate_basic_with_template(force, template).await?;

        // Generate Node.js-specific configs
        self.generate_nvmrc(force).await?;
        self.generate_eslint_config(force, template).await?;
        self.generate_node_package_json(force, template).await?;

        // Overwrite the basic justfile with Node.js-specific one
        self.generate_node_justfile(force, template).await?;

        Ok(())
    }

    pub async fn dry_run_node_with_template(&self, template: &str) -> Result<(), ZackstrapError> {
        println!("  📝 Would generate .editorconfig");
        println!("  🎨 Would generate .prettierrc (template: {})", template);
        println!("  🔧 Would generate justfile");
        println!("  🟢 Would generate .nvmrc (20)");
        println!("  🔍 Would generate .eslintrc.js (template: {})", template);
        println!("  📦 Would generate package.json (template: {})", template);
        println!(
            "  🔧 Would generate Node.js justfile (template: {})",
            template
        );
        Ok(())
    }

    // Go project generation
    pub async fn generate_go_with_template(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        // Generate basic configs first
        self.generate_basic_with_template(force, template).await?;

        // Generate Go-specific configs
        self.generate_go_mod(force).await?;
        self.generate_golangci_config(force).await?;
        self.generate_go_gitignore(force).await?;

        // Overwrite the basic justfile with Go-specific one
        self.generate_go_justfile(force, template).await?;

        Ok(())
    }

    pub async fn dry_run_go_with_template(&self, template: &str) -> Result<(), ZackstrapError> {
        println!("  📝 Would generate .editorconfig");
        println!("  🎨 Would generate .prettierrc (template: {})", template);
        println!("  🔧 Would generate justfile");
        println!("  🦀 Would generate go.mod");
        println!("  🔍 Would generate .golangci.yml");
        println!("  🚫 Would generate .gitignore additions");
        println!("  🔧 Would generate Go justfile (template: {})", template);
        Ok(())
    }

    // Rust project generation
    pub async fn generate_rust_with_template(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        // Generate basic configs first
        self.generate_basic_with_template(force, template).await?;

        // Generate Rust-specific configs
        self.generate_rustfmt_config(force).await?;
        self.generate_clippy_config(force).await?;
        self.generate_cargo_config(force).await?;

        // Overwrite the basic justfile with Rust-specific one
        self.generate_rust_justfile(force, template).await?;

        Ok(())
    }

    pub async fn dry_run_rust_with_template(&self, template: &str) -> Result<(), ZackstrapError> {
        println!("  📝 Would generate .editorconfig");
        println!("  📝 Would generate .prettierrc (template: {})", template);
        println!("  🔧 Would generate justfile");
        println!("  🦀 Would generate rustfmt.toml");
        println!("  🔍 Would generate .clippy.toml");
        println!("  ⚙️  Would generate .cargo/config.toml");
        println!("  🔧 Would generate Rust justfile (template: {})", template);
        Ok(())
    }

    pub async fn detect_project_type(&self) -> Result<ProjectType, ZackstrapError> {
        // Check for Ruby project indicators
        let gemfile = self.target_dir.join("Gemfile");
        let rakefile = self.target_dir.join("Rakefile");

        if gemfile.exists() || self.has_files_with_pattern("*.gemspec") || rakefile.exists() {
            return Ok(ProjectType::Ruby);
        }

        // Check for Python project indicators
        if self.has_files_with_pattern("*.py")
            || self.has_files_with_pattern("requirements*.txt")
            || self.has_files_with_pattern("pyproject.toml")
        {
            return Ok(ProjectType::Python);
        }

        // Check for Node.js project indicators
        if self.has_files_with_pattern("package.json")
            || self.has_files_with_pattern("*.js")
            || self.has_files_with_pattern("*.ts")
        {
            return Ok(ProjectType::Node);
        }

        // Check for Go project indicators
        if self.has_files_with_pattern("*.go")
            || self.has_files_with_pattern("go.mod")
            || self.has_files_with_pattern("go.sum")
        {
            return Ok(ProjectType::Go);
        }

        // Check for Rust project indicators
        if self.has_files_with_pattern("*.rs")
            || self.has_files_with_pattern("Cargo.toml")
            || self.has_files_with_pattern("Cargo.lock")
        {
            return Ok(ProjectType::Rust);
        }

        // Default to basic project
        Ok(ProjectType::Basic)
    }

    fn has_files_with_pattern(&self, pattern: &str) -> bool {
        if let Ok(entries) = std::fs::read_dir(&self.target_dir) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if glob::Pattern::new(pattern).is_ok_and(|p| p.matches(file_name)) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub async fn interactive_setup(&self) -> Result<(), ZackstrapError> {
        use std::io::{self, Write};

        println!("🎯 Welcome to Zackstrap Interactive Setup!");
        println!("I'll help you configure your project step by step.\n");

        // Ask for project type
        print!("What type of project is this? [basic/ruby/python/node/go/rust]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let project_type = input.trim().to_lowercase();

        // Ask for force overwrite
        print!("Force overwrite existing files? [y/N]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let force = input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes";

        println!();

        match project_type.as_str() {
            "ruby" => {
                println!("💎 Setting up Ruby project configuration...");
                self.generate_ruby(force).await?;
                println!("✅ Ruby configuration complete!");
            }
            "python" => {
                println!("🐍 Setting up Python project configuration...");
                self.generate_python_with_template(force, "default").await?;
                println!("✅ Python configuration complete!");
            }
            "node" => {
                println!("🟢 Setting up Node.js project configuration...");
                self.generate_node_with_template(force, "default").await?;
                println!("✅ Node.js configuration complete!");
            }
            "go" => {
                println!("🦀 Setting up Go project configuration...");
                self.generate_go_with_template(force, "default").await?;
                println!("✅ Go configuration complete!");
            }
            "rust" => {
                println!("🦀 Setting up Rust project configuration...");
                self.generate_rust_with_template(force, "default").await?;
                println!("✅ Rust configuration complete!");
            }
            "basic" => {
                println!("📁 Setting up basic project configuration...");
                self.generate_basic(force).await?;
                println!("✅ Basic configuration complete!");
            }
            _ => {
                println!("📁 Setting up basic project configuration...");
                self.generate_basic(force).await?;
                println!("✅ Basic configuration complete!");
            }
        }

        Ok(())
    }

    async fn generate_editor_config(&self, force: bool) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join(".editorconfig");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let config = EditorConfig::default();
        let content = self.editor_config_to_string(&config);

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  📝 Generated .editorconfig");
        Ok(())
    }

    async fn generate_prettier_config_with_template(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join(".prettierrc");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let config = match template {
            "google" => PrettierConfig {
                semi: true,
                single_quote: false,
                tab_width: 2,
                trailing_comma: "es5".to_string(),
                print_width: 80,
            },
            "airbnb" => PrettierConfig {
                semi: true,
                single_quote: true,
                tab_width: 2,
                trailing_comma: "es5".to_string(),
                print_width: 100,
            },
            _ => PrettierConfig::default(),
        };

        let content =
            serde_json::to_string_pretty(&config).map_err(ZackstrapError::SerializationError)?;

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🎨 Generated .prettierrc (template: {})", template);
        Ok(())
    }

    async fn generate_ruby_version(&self, force: bool) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join(".ruby-version");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        fs::write(&file_path, "3.3.0")
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  💎 Generated .ruby-version (3.3.0)");
        Ok(())
    }

    async fn generate_node_version(&self, force: bool) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join(".node-version");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        fs::write(&file_path, "24")
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🟢 Generated .node-version (24)");
        Ok(())
    }

    async fn generate_rubocop_config_with_template(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join(".rubocop.yml");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let content = match template {
            "rails" => self.get_rubocop_rails_content(),
            "sinatra" => self.get_rubocop_sinatra_content(),
            "gem" => self.get_rubocop_gem_content(),
            _ => self.get_rubocop_content(),
        };

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🔍 Generated .rubocop.yml (template: {})", template);
        Ok(())
    }

    async fn generate_package_json_with_template(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join("package.json");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let config = match template {
            "rails" => PackageJson {
                name: "rails-app".to_string(),
                version: "0.1.0".to_string(),
                description: "A Ruby on Rails application".to_string(),
                dev_dependencies: {
                    let mut deps = std::collections::HashMap::new();
                    deps.insert("prettier".to_string(), "^3.0.0".to_string());
                    deps.insert(
                        "prettier-plugin-ruby".to_string(),
                        "github:prettier/plugin-ruby".to_string(),
                    );
                    deps.insert("eslint".to_string(), "^8.0.0".to_string());
                    deps
                },
            },
            "sinatra" => PackageJson {
                name: "sinatra-app".to_string(),
                version: "0.1.0".to_string(),
                description: "A Sinatra application".to_string(),
                dev_dependencies: {
                    let mut deps = std::collections::HashMap::new();
                    deps.insert("prettier".to_string(), "^3.0.0".to_string());
                    deps.insert(
                        "prettier-plugin-ruby".to_string(),
                        "github:prettier/plugin-ruby".to_string(),
                    );
                    deps
                },
            },
            "gem" => PackageJson {
                name: "ruby-gem".to_string(),
                version: "0.1.0".to_string(),
                description: "A Ruby gem".to_string(),
                dev_dependencies: {
                    let mut deps = std::collections::HashMap::new();
                    deps.insert("prettier".to_string(), "^3.0.0".to_string());
                    deps.insert(
                        "prettier-plugin-ruby".to_string(),
                        "github:prettier/plugin-ruby".to_string(),
                    );
                    deps.insert("rspec".to_string(), "^3.12".to_string());
                    deps
                },
            },
            _ => PackageJson::default(),
        };

        let content =
            serde_json::to_string_pretty(&config).map_err(ZackstrapError::SerializationError)?;

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  📦 Generated package.json (template: {})", template);
        Ok(())
    }

    fn editor_config_to_string(&self, _config: &EditorConfig) -> String {
        let mut content = String::new();

        // Root declaration should be at the top
        content.push_str("root = true\n");
        content.push('\n');

        // Default settings for all files
        content.push_str("# Default settings for all files\n");
        content.push_str("[*]\n");
        content.push_str("charset = utf-8\n");
        content.push_str("end_of_line = lf\n");
        content.push_str("indent_style = space\n");
        content.push_str("indent_size = 2\n");
        content.push_str("insert_final_newline = true\n");
        content.push_str("trim_trailing_whitespace = true\n");
        content.push('\n');

        // Windows batch files need CRLF
        content.push_str("# Windows batch files need CRLF\n");
        content.push_str("[*.bat]\n");
        content.push_str("end_of_line = crlf\n");
        content.push('\n');

        // Python files - typically use 4 spaces
        content.push_str("# Python files - typically use 4 spaces\n");
        content.push_str("[*.{py,pyw,ipynb}]\n");
        content.push_str("indent_size = 4\n");
        content.push('\n');

        // PHP files - typically use 4 spaces
        content.push_str("# PHP files - typically use 4 spaces\n");
        content.push_str("[*.{php,phtml,phps}]\n");
        content.push_str("indent_size = 4\n");
        content.push('\n');

        // Rust files - typically use 4 spaces
        content.push_str("# Rust files - typically use 4 spaces\n");
        content.push_str("[*.{rs,toml}]\n");
        content.push_str("indent_size = 4\n");
        content.push('\n');

        // Elixir files
        content.push_str("# Elixir files\n");
        content.push_str("[*.{ex,exs,eex,heex,leex}]\n");
        content.push_str("indent_size = 2\n");
        content.push('\n');

        // Ruby and related files (already 2 spaces from default)
        content.push_str("# Ruby and related files (already 2 spaces from default)\n");
        content.push_str("[*.{rb,erb,ru,rake,gemspec}]\n");
        content.push_str("indent_size = 2\n");
        content.push('\n');

        // Web and configuration files (already 2 spaces from default)
        content.push_str("# Web and configuration files (already 2 spaces from default)\n");
        content.push_str("[*.{yml,yaml,haml,jbuilder,jsx,html,sls,tf}]\n");
        content.push_str("indent_size = 2\n");
        content.push('\n');

        // Makefiles require tabs
        content.push_str("# Makefiles require tabs\n");
        content.push_str("[{*[Mm]akefile*,*.mak,*.mk,depend}]\n");
        content.push_str("indent_style = tab\n");
        content.push('\n');

        // Specific directories
        content.push_str("# Specific directories\n");
        content.push_str("[enc/*]\n");
        content.push_str("indent_size = 2\n");
        content.push('\n');
        content.push_str("[reg*.[ch]]\n");
        content.push_str("indent_size = 2\n");

        content
    }

    fn get_rubocop_rails_content(&self) -> String {
        r#"# RuboCop configuration for Rails projects
# This is a Rails-specific configuration

AllCops:
  NewCops: enable
  TargetRubyVersion: 3.3
  Exclude:
    - 'vendor/**/*'
    - 'node_modules/**/*'
    - 'db/schema.rb'
    - 'config/application.rb'
    - 'config/environments/**/*'
    - 'config/initializers/**/*'
    - 'config/routes.rb'
    - 'Gemfile'
    - 'Rakefile'
    - 'bin/**/*'
    - 'lib/tasks/**/*'

# Rails-specific rules
Rails:
  Enabled: true

Rails/FilePath:
  Enabled: true

Rails/TimeZone:
  Enabled: true

Rails/Date:
  Enabled: true

# Layout
Layout/LineLength:
  Max: 120

Layout/IndentationWidth:
  Width: 2

# Style
Style/Documentation:
  Enabled: false

Style/FrozenStringLiteralComment:
  Enabled: false

Style/StringLiterals:
  EnforcedStyle: single_quotes

# Metrics
Metrics/BlockLength:
  Exclude:
    - 'spec/**/*'
    - 'test/**/*'
    - 'config/routes.rb'

Metrics/MethodLength:
  Max: 20

Metrics/AbcSize:
  Max: 30
"#
        .to_string()
    }

    fn get_rubocop_sinatra_content(&self) -> String {
        r#"# RuboCop configuration for Sinatra projects
# This is a Sinatra-specific configuration

AllCops:
  NewCops: enable
  TargetRubyVersion: 3.3
  Exclude:
    - 'vendor/**/*'
    - 'node_modules/**/*'
    - 'Gemfile'
    - 'Rakefile'

# Layout
Layout/LineLength:
  Max: 120

Layout/IndentationWidth:
  Width: 2

# Style
Style/Documentation:
  Enabled: false

Style/FrozenStringLiteralComment:
  Enabled: false

Style/StringLiterals:
  EnforcedStyle: single_quotes

# Metrics
Metrics/MethodLength:
  Max: 25

Metrics/AbcSize:
  Max: 35
"#
        .to_string()
    }

    fn get_rubocop_gem_content(&self) -> String {
        r#"# RuboCop configuration for Ruby gems
# This is a gem-specific configuration

AllCops:
  NewCops: enable
  TargetRubyVersion: 3.3
  Exclude:
    - 'vendor/**/*'
    - 'node_modules/**/*'
    - 'Gemfile'
    - 'Rakefile'
    - '*.gemspec'

# Layout
Layout/LineLength:
  Max: 120

Layout/IndentationWidth:
  Width: 2

# Style
Style/Documentation:
  Enabled: true

Style/FrozenStringLiteralComment:
  Enabled: true

Style/StringLiterals:
  EnforcedStyle: single_quotes

# Metrics
Metrics/MethodLength:
  Max: 15

Metrics/AbcSize:
  Max: 25

# RSpec
RSpec:
  Exclude:
    - 'spec/spec_helper.rb'

RSpec/ExampleLength:
  Max: 15

RSpec/MultipleExpectations:
  Max: 3
"#
        .to_string()
    }

    fn get_rubocop_content(&self) -> String {
        r#"# RuboCop configuration file
# This is a comprehensive configuration for Ruby projects

AllCops:
  NewCops: enable
  TargetRubyVersion: 3.3
  Exclude:
    - 'vendor/**/*'
    - 'node_modules/**/*'
    - 'db/schema.rb'
    - 'config/application.rb'
    - 'config/environments/**/*'
    - 'config/initializers/**/*'
    - 'config/routes.rb'
    - 'Gemfile'
    - 'Rakefile'

# Layout
Layout/LineLength:
  Max: 120
  Exclude:
    - 'spec/**/*'
    - 'test/**/*'

Layout/IndentationWidth:
  Width: 2

Layout/IndentHash:
  EnforcedStyle: consistent

# Lint
Lint/UnusedMethodArgument:
  Enabled: true

Lint/UnusedBlockArgument:
  Enabled: true

Lint/UnreachableCode:
  Enabled: true

# Style
Style/Documentation:
  Enabled: false

Style/FrozenStringLiteralComment:
  Enabled: false

Style/StringLiterals:
  EnforcedStyle: single_quotes

Style/SymbolArray:
  EnforcedStyle: percent
  MinSize: 3

Style/WordArray:
  EnforcedStyle: percent
  MinSize: 3

Style/IfUnlessModifier:
  Enabled: true

Style/WhileUntilModifier:
  Enabled: true

Style/GuardClause:
  Enabled: true

# Metrics
Metrics/BlockLength:
  Exclude:
    - 'spec/**/*'
    - 'test/**/*'
    - 'config/routes.rb'

Metrics/MethodLength:
  Max: 20

Metrics/AbcSize:
  Max: 30

Metrics/ClassLength:
  Max: 150

Metrics/ModuleLength:
  Max: 150

# Performance
Performance/StringReplacement:
  Enabled: true

Performance/RedundantMatch:
  Enabled: true

Performance/RedundantMerge:
  Enabled: true

# Security
Security/YAMLLoad:
  Enabled: true

Security/JSONLoad:
  Enabled: true

# Bundler
Bundler/OrderedGems:
  Enabled: true

Bundler/DuplicatedGroup:
  Enabled: true

# RSpec
RSpec:
  Exclude:
    - 'spec/rails_helper.rb'
    - 'spec/spec_helper.rb'

RSpec/ExampleLength:
  Max: 20

RSpec/DescribedClass:
  Enabled: true

RSpec/InstanceVariable:
  Enabled: true

RSpec/LetSetup:
  Enabled: true

RSpec/MultipleExpectations:
  Max: 5

RSpec/NestedGroups:
  Max: 4

RSpec/ReturnFromStub:
  Enabled: true

RSpec/StubbedMock:
  Enabled: true

RSpec/SubjectDeclaration:
  Enabled: true

RSpec/VerifiedDoubles:
  Enabled: true
"#
        .to_string()
    }

    async fn generate_justfile(&self, force: bool) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join("justfile");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let content = self.get_justfile_content();

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🔧 Generated justfile");
        Ok(())
    }

    fn get_justfile_content(&self) -> String {
        r#"# Project Justfile
# Just is a command runner - https://github.com/casey/just

# Default target
default:
    @just --list

# Build the project
build:
    cargo build

# Build release version
release:
    cargo build --release

# Check for compilation errors
check:
    cargo check

# Run tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run clippy linter
clippy:
    cargo clippy

# Run clippy with all warnings
clippy-strict:
    cargo clippy -- -D warnings

# Format code
fmt:
    cargo fmt

# Check formatting
fmt-check:
    cargo fmt -- --check

# Clean build artifacts
clean:
    cargo clean

# Install the binary globally
install:
    cargo install --path .

# Uninstall the binary
uninstall:
    cargo uninstall

# Show help
help:
    @just --list

# Test all features
test-all: check test clippy fmt-check

# Prepare for release
prepare-release: test-all release

# Generate documentation
doc:
    cargo doc --open

# Check for security vulnerabilities
audit:
    cargo audit

# Update dependencies
update:
    cargo update

# Show outdated dependencies
outdated:
    cargo outdated

# Install development tools
install-tools:
    cargo install cargo-audit
    cargo install cargo-outdated
    cargo install cargo-watch

# Watch for changes and run tests
watch:
    cargo watch -x check -x test

# Run with cargo watch
dev:
    cargo watch -x run

# Show project info
info:
    @echo "Project Information"
    @echo "=================="
    @echo "Just version: $(just --version)"
    @echo "Cargo version: $(cargo --version)"
    @echo "Rust version: $(rustc --version)"
"#
        .to_string()
    }

    async fn generate_ruby_justfile(
        &self,
        _force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join("justfile");

        // For Ruby projects, always overwrite the justfile since it's meant to replace the basic one
        let content = self.get_ruby_justfile_content(template);

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🔧 Generated Ruby justfile (template: {})", template);
        Ok(())
    }

    pub fn get_ruby_justfile_content(&self, template: &str) -> String {
        let mut content = String::new();

        content.push_str("# Ruby Project Justfile\n");
        content.push_str("# Just is a command runner - https://github.com/casey/just\n\n");

        // Common Rust commands
        content.push_str("# Build and test\n");
        content.push_str("build:\n    cargo build\n\n");
        content.push_str("release:\n    cargo build --release\n\n");
        content.push_str("check:\n    cargo check\n\n");
        content.push_str("test:\n    cargo test\n\n");
        content.push_str("clippy:\n    cargo clippy\n\n");
        content.push_str("fmt:\n    cargo fmt\n\n");
        content.push_str("clean:\n    cargo clean\n\n");

        // Ruby-specific commands
        content.push_str("# Ruby development\n");
        content.push_str("ruby-version:\n    ruby --version\n\n");
        content.push_str("bundle-install:\n    bundle install\n\n");
        content.push_str("bundle-update:\n    bundle update\n\n");

        match template {
            "rails" => {
                content.push_str("# Rails-specific commands\n");
                content.push_str("rails-server:\n    rails server\n\n");
                content.push_str("rails-console:\n    rails console\n\n");
                content.push_str("rails-routes:\n    rails routes\n\n");
                content.push_str("rails-db-migrate:\n    rails db:migrate\n\n");
                content.push_str("rails-db-seed:\n    rails db:seed\n\n");
                content.push_str("rails-test:\n    rails test\n\n");
                content.push_str("rails-spec:\n    bundle exec rspec\n\n");
            }
            "sinatra" => {
                content.push_str("# Sinatra-specific commands\n");
                content.push_str("sinatra-server:\n    sinatra app.rb\n\n");
                content.push_str("sinatra-console:\n    bundle exec irb -r ./app.rb\n\n");
                content.push_str("sinatra-test:\n    bundle exec rspec\n\n");
            }
            "gem" => {
                content.push_str("# Gem development commands\n");
                content.push_str("gem-build:\n    gem build *.gemspec\n\n");
                content.push_str("gem-install:\n    gem install *.gem\n\n");
                content.push_str("gem-test:\n    bundle exec rspec\n\n");
                content.push_str("gem-release:\n    bundle exec rake release\n\n");
            }
            _ => {
                content.push_str("# Ruby development commands\n");
                content.push_str("ruby-run:\n    ruby main.rb\n\n");
                content.push_str("ruby-test:\n    bundle exec rspec\n\n");
            }
        }

        // Common development commands
        content.push_str("# Development tools\n");
        content.push_str("install-tools:\n    gem install rubocop bundler\n\n");
        content.push_str("rubocop:\n    bundle exec rubocop\n\n");
        content.push_str("rubocop-fix:\n    bundle exec rubocop -a\n\n");
        content.push_str("prettier:\n    npx prettier --write .\n\n");

        // Project info
        content.push_str("# Project info\n");
        content.push_str("info:\n");
        content.push_str("    @echo \"Ruby Project Information\"\n");
        content.push_str("    @echo \"========================\"\n");
        content.push_str("    @echo \"Ruby version: $(ruby --version)\"\n");
        content.push_str("    @echo \"Bundler version: $(bundle --version)\"\n");
        content.push_str("    @echo \"Just version: $(just --version)\"\n");

        content
    }

    // Python file generation methods
    async fn generate_python_version(&self, force: bool) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join(".python-version");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let content = "3.12\n";

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🐍 Generated .python-version");
        Ok(())
    }

    async fn generate_pyproject_toml(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join("pyproject.toml");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let content = self.get_pyproject_toml_content(template);

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  📦 Generated pyproject.toml (template: {})", template);
        Ok(())
    }

    async fn generate_flake8_config(&self, force: bool) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join(".flake8");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let content = "[flake8]\nmax-line-length = 88\nextend-ignore = E203, W503\n";

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🔍 Generated .flake8");
        Ok(())
    }

    async fn generate_requirements_dev(&self, force: bool) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join("requirements-dev.txt");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let content = "black==23.12.1\nflake8==7.0.1\nmypy==1.8.0\npytest==7.4.4\n";

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  📋 Generated requirements-dev.txt");
        Ok(())
    }

    async fn generate_python_justfile(
        &self,
        _force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join("justfile");

        // For Python projects, always overwrite the justfile since it's meant to replace the basic one
        let content = self.get_python_justfile_content(template);

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🔧 Generated Python justfile (template: {})", template);
        Ok(())
    }

    // Node.js file generation methods
    async fn generate_nvmrc(&self, force: bool) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join(".nvmrc");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let content = "20\n";

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🟢 Generated .nvmrc");
        Ok(())
    }

    async fn generate_eslint_config(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join(".eslintrc.js");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let content = self.get_eslint_config_content(template);

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🔍 Generated .eslintrc.js (template: {})", template);
        Ok(())
    }

    async fn generate_node_package_json(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join("package.json");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let content = self.get_node_package_json_content(template);

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  📦 Generated package.json (template: {})", template);
        Ok(())
    }

    async fn generate_node_justfile(
        &self,
        _force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join("justfile");

        // For Node.js projects, always overwrite the justfile since it's meant to replace the basic one

        let content = self.get_node_justfile_content(template);

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🔧 Generated Node.js justfile (template: {})", template);
        Ok(())
    }

    // Go file generation methods
    async fn generate_go_mod(&self, force: bool) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join("go.mod");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let content = "module myproject\n\ngo 1.21\n";

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🦀 Generated go.mod");
        Ok(())
    }

    async fn generate_golangci_config(&self, force: bool) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join(".golangci.yml");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let content = "run:\n  timeout: 5m\n\nlinters:\n  enable:\n    - gofmt\n    - golint\n    - govet\n    - errcheck\n";

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🔍 Generated .golangci.yml");
        Ok(())
    }

    async fn generate_go_gitignore(&self, force: bool) -> Result<(), ZackstrapError> {
        let gitignore_path = self.target_dir.join(".gitignore");

        let go_gitignore_content =
            "\n# Go\n*.exe\n*.exe~\n*.dll\n*.so\n*.dylib\n*.test\n*.out\ngo.work\n";

        if gitignore_path.exists() {
            if !force {
                return Err(ZackstrapError::FileExists(gitignore_path));
            }
            // Append to existing .gitignore
            let mut existing_content = fs::read_to_string(&gitignore_path)
                .map_err(|e| ZackstrapError::WriteFileError(gitignore_path.clone(), e))?;
            existing_content.push_str(go_gitignore_content);
            fs::write(&gitignore_path, existing_content)
                .map_err(|e| ZackstrapError::WriteFileError(gitignore_path.clone(), e))?;
        } else {
            // Create new .gitignore
            fs::write(&gitignore_path, go_gitignore_content)
                .map_err(|e| ZackstrapError::WriteFileError(gitignore_path.clone(), e))?;
        }

        println!("  🚫 Updated .gitignore with Go patterns");
        Ok(())
    }

    async fn generate_go_justfile(
        &self,
        _force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join("justfile");

        // For Go projects, always overwrite the justfile since it's meant to replace the basic one

        let content = self.get_go_justfile_content(template);

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🔧 Generated Go justfile (template: {})", template);
        Ok(())
    }

    // Rust file generation methods
    async fn generate_rustfmt_config(&self, force: bool) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join("rustfmt.toml");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let content = "edition = \"2021\"\nmax_width = 100\n";

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🦀 Generated rustfmt.toml");
        Ok(())
    }

    async fn generate_clippy_config(&self, force: bool) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join(".clippy.toml");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let content = "# Clippy configuration\n";

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🔍 Generated .clippy.toml");
        Ok(())
    }

    async fn generate_cargo_config(&self, force: bool) -> Result<(), ZackstrapError> {
        let cargo_dir = self.target_dir.join(".cargo");
        let file_path = cargo_dir.join("config.toml");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        // Create .cargo directory if it doesn't exist
        if !cargo_dir.exists() {
            fs::create_dir_all(&cargo_dir)
                .map_err(|e| ZackstrapError::WriteFileError(cargo_dir.clone(), e))?;
        }

        let content = "[build]\nrustflags = [\"-C\", \"target-cpu=native\"]\n";

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  ⚙️  Generated .cargo/config.toml");
        Ok(())
    }

    async fn generate_rust_justfile(
        &self,
        _force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join("justfile");

        // For Rust projects, always overwrite the justfile since it's meant to replace the basic one

        let content = self.get_rust_justfile_content(template);

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🔧 Generated Rust justfile (template: {})", template);
        Ok(())
    }

    // Content generation methods for new languages
    pub fn get_pyproject_toml_content(&self, template: &str) -> String {
        let mut content = String::new();
        content.push_str("[tool.black]\n");
        content.push_str("line-length = 88\n");
        content.push_str("target-version = ['py312']\n\n");
        content.push_str("[tool.flake8]\n");
        content.push_str("max-line-length = 88\n");
        content.push_str("extend-ignore = [\"E203\", \"W503\"]\n\n");
        content.push_str("[tool.mypy]\n");
        content.push_str("python_version = \"3.12\"\n");
        content.push_str("strict = true\n\n");
        content.push_str("[tool.pytest.ini_options]\n");
        content.push_str("testpaths = [\"tests\"]\n");
        content.push_str("python_files = [\"test_*.py\"]\n");

        match template {
            "django" => {
                content.push_str("\n[tool.django-stubs]\n");
                content.push_str("django_settings_module = \"myproject.settings\"\n");
            }
            "flask" => {
                content.push_str("\n[tool.flask]\n");
                content.push_str("app_name = \"app\"\n");
            }
            _ => {}
        }

        content
    }

    pub fn get_eslint_config_content(&self, template: &str) -> String {
        let mut content = String::new();
        content.push_str("module.exports = {\n");
        content.push_str("  env: {\n");
        content.push_str("    node: true,\n");
        content.push_str("    es2022: true,\n");
        content.push_str("  },\n");
        content.push_str("  extends: [\n");
        content.push_str("    'eslint:recommended',\n");
        content.push_str("  ],\n");
        content.push_str("  parserOptions: {\n");
        content.push_str("    ecmaVersion: 2022,\n");
        content.push_str("    sourceType: 'module',\n");
        content.push_str("  },\n");

        match template {
            "express" => {
                content.push_str("  rules: {\n");
                content.push_str("    'no-console': 'off',\n");
                content.push_str("  },\n");
            }
            "react" => {
                content.push_str("  extends: [\n");
                content.push_str("    'eslint:recommended',\n");
                content.push_str("    'plugin:react/recommended',\n");
                content.push_str("  ],\n");
                content.push_str("  plugins: ['react'],\n");
                content.push_str("  rules: {\n");
                content.push_str("    'react/prop-types': 'off',\n");
                content.push_str("  },\n");
            }
            _ => {}
        }

        content.push_str("};\n");
        content
    }

    pub fn get_node_package_json_content(&self, template: &str) -> String {
        let mut content = String::new();
        content.push_str("{\n");
        content.push_str("  \"name\": \"myproject\",\n");
        content.push_str("  \"version\": \"1.0.0\",\n");
        content.push_str("  \"description\": \"A Node.js project\",\n");
        content.push_str("  \"main\": \"index.js\",\n");
        content.push_str("  \"scripts\": {\n");
        content.push_str("    \"test\": \"node test.js\",\n");
        content.push_str("    \"lint\": \"eslint .\",\n");
        content.push_str("    \"format\": \"prettier --write .\"\n");
        content.push_str("  },\n");
        content.push_str("  \"devDependencies\": {\n");
        content.push_str("    \"eslint\": \"^8.57.0\",\n");
        content.push_str("    \"prettier\": \"^3.2.5\"\n");

        match template {
            "express" => {
                content.push_str(",\n    \"express\": \"^4.18.2\"\n");
            }
            "react" => {
                content
                    .push_str(",\n    \"react\": \"^18.2.0\",\n    \"react-dom\": \"^18.2.0\"\n");
            }
            _ => {}
        }

        content.push_str("\n  }\n}\n");
        content
    }

    pub fn get_python_justfile_content(&self, template: &str) -> String {
        let mut content = String::new();
        content.push_str("# Python Project Justfile\n");
        content.push_str("# Just is a command runner - https://github.com/casey/just\n\n");
        content.push_str("# Python development\n");
        content.push_str("python-version:\n    python --version\n\n");
        content.push_str("install-dev:\n    pip install -r requirements-dev.txt\n\n");
        content.push_str("format:\n    black .\n\n");
        content.push_str("lint:\n    flake8 .\n\n");
        content.push_str("type-check:\n    mypy .\n\n");
        content.push_str("test:\n    pytest\n\n");

        match template {
            "django" => {
                content.push_str("# Django-specific commands\n");
                content.push_str("django-server:\n    python manage.py runserver\n\n");
                content.push_str("django-migrate:\n    python manage.py migrate\n\n");
                content.push_str("django-shell:\n    python manage.py shell\n\n");
            }
            "flask" => {
                content.push_str("# Flask-specific commands\n");
                content.push_str("flask-run:\n    flask run\n\n");
                content.push_str("flask-shell:\n    flask shell\n\n");
            }
            _ => {}
        }

        content.push_str("# Project info\n");
        content.push_str("info:\n");
        content.push_str("    @echo \"Python Project Information\"\n");
        content.push_str("    @echo \"==========================\"\n");
        content.push_str("    @echo \"Python version: $(python --version)\"\n");
        content.push_str("    @echo \"Just version: $(just --version)\"\n");

        content
    }

    pub fn get_node_justfile_content(&self, template: &str) -> String {
        let mut content = String::new();
        content.push_str("# Node.js Project Justfile\n");
        content.push_str("# Just is a command runner - https://github.com/casey/just\n\n");
        content.push_str("# Node.js development\n");
        content.push_str("node-version:\n    node --version\n\n");
        content.push_str("npm-install:\n    npm install\n\n");
        content.push_str("npm-test:\n    npm test\n\n");
        content.push_str("npm-lint:\n    npm run lint\n\n");
        content.push_str("npm-format:\n    npm run format\n\n");

        match template {
            "express" => {
                content.push_str("# Express-specific commands\n");
                content.push_str("express-server:\n    node app.js\n\n");
                content.push_str("express-dev:\n    nodemon app.js\n\n");
            }
            "react" => {
                content.push_str("# React-specific commands\n");
                content.push_str("react-dev:\n    npm start\n\n");
                content.push_str("react-build:\n    npm run build\n\n");
            }
            _ => {}
        }

        content.push_str("# Project info\n");
        content.push_str("info:\n");
        content.push_str("    @echo \"Node.js Project Information\"\n");
        content.push_str("    @echo \"==========================\"\n");
        content.push_str("    @echo \"Node version: $(node --version)\"\n");
        content.push_str("    @echo \"NPM version: $(npm --version)\"\n");
        content.push_str("    @echo \"Just version: $(just --version)\"\n");

        content
    }

    pub fn get_go_justfile_content(&self, template: &str) -> String {
        let mut content = String::new();
        content.push_str("# Go Project Justfile\n");
        content.push_str("# Just is a command runner - https://github.com/casey/just\n\n");
        content.push_str("# Go development\n");
        content.push_str("go-version:\n    go version\n\n");
        content.push_str("go-build:\n    go build\n\n");
        content.push_str("go-run:\n    go run .\n\n");
        content.push_str("go-test:\n    go test ./...\n\n");
        content.push_str("go-lint:\n    golangci-lint run\n\n");
        content.push_str("go-fmt:\n    go fmt ./...\n\n");
        content.push_str("go-mod-tidy:\n    go mod tidy\n\n");

        match template {
            "web" => {
                content.push_str("# Web-specific commands\n");
                content.push_str("go-web:\n    go run cmd/web/main.go\n\n");
            }
            "cli" => {
                content.push_str("# CLI-specific commands\n");
                content.push_str("go-cli:\n    go run cmd/cli/main.go\n\n");
            }
            _ => {}
        }

        content.push_str("# Project info\n");
        content.push_str("info:\n");
        content.push_str("    @echo \"Go Project Information\"\n");
        content.push_str("    @echo \"=====================\"\n");
        content.push_str("    @echo \"Go version: $(go version)\"\n");
        content.push_str("    @echo \"Just version: $(just --version)\"\n");

        content
    }

    pub fn get_rust_justfile_content(&self, template: &str) -> String {
        let mut content = String::new();
        content.push_str("# Rust Project Justfile\n");
        content.push_str("# Just is a command runner - https://github.com/casey/just\n\n");
        content.push_str("# Rust development\n");
        content.push_str("rust-version:\n    rustc --version\n\n");
        content.push_str("cargo-build:\n    cargo build\n\n");
        content.push_str("cargo-run:\n    cargo run\n\n");
        content.push_str("cargo-test:\n    cargo test\n\n");
        content.push_str("cargo-clippy:\n    cargo clippy\n\n");
        content.push_str("cargo-fmt:\n    cargo fmt\n\n");
        content.push_str("cargo-check:\n    cargo check\n\n");

        match template {
            "web" => {
                content.push_str("# Web-specific commands\n");
                content.push_str("cargo-web:\n    cargo run --bin web\n\n");
            }
            "cli" => {
                content.push_str("# CLI-specific commands\n");
                content.push_str("cargo-cli:\n    cargo run --bin cli\n\n");
            }
            _ => {}
        }

        content.push_str("# Project info\n");
        content.push_str("info:\n");
        content.push_str("    @echo \"Rust Project Information\"\n");
        content.push_str("    @echo \"========================\"\n");
        content.push_str("    @echo \"Rust version: $(rustc --version)\"\n");
        content.push_str("    @echo \"Cargo version: $(cargo --version)\"\n");
        content.push_str("    @echo \"Just version: $(just --version)\"\n");

        content
    }
}
