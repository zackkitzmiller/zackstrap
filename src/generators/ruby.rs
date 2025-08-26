use crate::config::PackageJson;
use crate::error::ZackstrapError;
use super::common::FileGenerator;

impl super::ConfigGenerator {
    pub async fn generate_ruby(&self, force: bool) -> Result<(), ZackstrapError> {
        self.generate_ruby_with_template(force, "default").await
    }

    pub async fn generate_ruby_with_template(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        // Generate basic configs first (includes justfile)
        self.generate_basic_with_template(force, false, template).await?;

        // Generate Ruby-specific configs
        self.generate_ruby_version(force).await?;
        self.generate_node_version(force).await?;
        self.generate_rubocop_config_with_template(force, template).await?;
        self.generate_package_json_with_template(force, template).await?;

        // Overwrite the basic justfile with Ruby-specific one
        self.generate_ruby_justfile(true, template).await?;

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

    async fn generate_ruby_version(&self, force: bool) -> Result<(), ZackstrapError> {
        let content = "3.3.0\n";
        self.write_file_if_not_exists(".ruby-version", content, force, false).await
    }

    async fn generate_node_version(&self, force: bool) -> Result<(), ZackstrapError> {
        let content = "24\n";
        self.write_file_if_not_exists(".node-version", content, force, false).await
    }

    async fn generate_rubocop_config_with_template(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let content = match template {
            "rails" => r#"# Rails-specific RuboCop configuration
inherit_from: .rubocop_todo.yml

AllCops:
  NewCops: enable
  TargetRubyVersion: 3.3
  Exclude:
    - 'db/schema.rb'
    - 'bin/**/*'
    - 'config/**/*'
    - 'vendor/**/*'

Style/Documentation:
  Enabled: false

Style/StringLiterals:
  EnforcedStyle: single_quotes

Layout/LineLength:
  Max: 120

Metrics/BlockLength:
  Exclude:
    - 'spec/**/*'
    - 'test/**/*'
"#,
            "sinatra" => r#"# Sinatra-specific RuboCop configuration
inherit_from: .rubocop_todo.yml

AllCops:
  NewCops: enable
  TargetRubyVersion: 3.3
  Exclude:
    - 'vendor/**/*'

Style/Documentation:
  Enabled: false

Style/StringLiterals:
  EnforcedStyle: single_quotes

Layout/LineLength:
  Max: 120
"#,
            "gem" => r#"# Gem-specific RuboCop configuration
inherit_from: .rubocop_todo.yml

AllCops:
  NewCops: enable
  TargetRubyVersion: 3.3
  Exclude:
    - 'vendor/**/*'

Style/Documentation:
  Enabled: false

Style/StringLiterals:
  EnforcedStyle: single_quotes

Layout/LineLength:
  Max: 120
"#,
            _ => r#"# Default RuboCop configuration
inherit_from: .rubocop_todo.yml

AllCops:
  NewCops: enable
  TargetRubyVersion: 3.3

Style/Documentation:
  Enabled: false

Style/StringLiterals:
  EnforcedStyle: single_quotes

Layout/LineLength:
  Max: 120
"#,
        };
        self.write_file_if_not_exists(".rubocop.yml", content, force, false).await
    }

    async fn generate_package_json_with_template(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let package_json = match template {
            "rails" | "sinatra" | "gem" => PackageJson::from_template(template),
            _ => PackageJson::default(),
        };
        let content = package_json.to_string();
        self.write_file_if_not_exists("package.json", &content, force, false).await
    }

    async fn generate_ruby_justfile(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let content = match template {
            "rails" => r#"# Rails project justfile
default:
    @echo "Available Rails commands:"
    @just --list

# Start Rails server
server:
    @echo "Starting Rails server..."
    @bundle exec rails server

# Run tests
test:
    @echo "Running Rails tests..."
    @bundle exec rails test

# Run RuboCop
rubocop:
    @echo "Running RuboCop..."
    @bundle exec rubocop

# Database operations
db:migrate:
    @echo "Running database migrations..."
    @bundle exec rails db:migrate

db:seed:
    @echo "Seeding database..."
    @bundle exec rails db:seed

# Install dependencies
install:
    @echo "Installing Ruby dependencies..."
    @bundle install
"#,
            "sinatra" => r#"# Sinatra project justfile
default:
    @echo "Available Sinatra commands:"
    @just --list

# Start Sinatra server
server:
    @echo "Starting Sinatra server..."
    @bundle exec ruby app.rb

# Run tests
test:
    @echo "Running Sinatra tests..."
    @bundle exec rspec

# Run RuboCop
rubocop:
    @echo "Running RuboCop..."
    @bundle exec rubocop

# Install dependencies
install:
    @echo "Installing Ruby dependencies..."
    @bundle install
"#,
            "gem" => r#"# Ruby gem project justfile
default:
    @echo "Available gem commands:"
    @just --list

# Build gem
build:
    @echo "Building gem..."
    @gem build *.gemspec

# Install gem locally
install:
    @echo "Installing gem locally..."
    @gem install *.gem

# Run tests
test:
    @echo "Running gem tests..."
    @bundle exec rspec

# Run RuboCop
rubocop:
    @echo "Running RuboCop..."
    @bundle exec rubocop

# Install dependencies
install-deps:
    @echo "Installing Ruby dependencies..."
    @bundle install
"#,
            _ => r#"# Ruby project justfile
default:
    @echo "Available Ruby commands:"
    @just --list

# Run tests
test:
    @echo "Running Ruby tests..."
    @bundle exec rspec

# Run RuboCop
rubocop:
    @echo "Running RuboCop..."
    @bundle exec rubocop

# Install dependencies
install:
    @echo "Installing Ruby dependencies..."
    @bundle install
"#,
        };
        self.write_file_if_not_exists("justfile", content, force, false).await
    }
}
