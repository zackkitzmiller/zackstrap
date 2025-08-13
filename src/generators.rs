use crate::config::{EditorConfig, PackageJson, PrettierConfig};
use crate::error::ZackstrapError;
use colored::*;
use std::fs;
use std::path::PathBuf;
use std::io::Write;

pub enum ProjectType {
    Basic,
    Ruby,
}

pub struct ConfigGenerator {
    target_dir: PathBuf,
}

impl ConfigGenerator {
    pub fn new(target_dir: PathBuf) -> Self {
        Self { target_dir }
    }

    pub async fn generate_basic(&self, force: bool) -> Result<(), ZackstrapError> {
        self.generate_editor_config(force).await?;
        self.generate_prettier_config(force).await?;
        Ok(())
    }

    pub async fn generate_ruby(&self, force: bool) -> Result<(), ZackstrapError> {
        // Generate basic configs first
        self.generate_basic(force).await?;

        // Generate Ruby-specific configs
        self.generate_ruby_version(force).await?;
        self.generate_node_version(force).await?;
        self.generate_rubocop_config(force).await?;
        self.generate_package_json(force).await?;

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

    async fn generate_prettier_config(&self, force: bool) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join(".prettierrc");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let config = PrettierConfig::default();
        let content = serde_json::to_string_pretty(&config)
            .map_err(ZackstrapError::SerializationError)?;

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🎨 Generated .prettierrc");
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

    async fn generate_rubocop_config(&self, force: bool) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join(".rubocop.yml");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let content = self.get_rubocop_content();

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  🔍 Generated .rubocop.yml");
        Ok(())
    }

    async fn generate_package_json(&self, force: bool) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir.join("package.json");

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        let config = PackageJson::default();
        let content = serde_json::to_string_pretty(&config)
            .map_err(ZackstrapError::SerializationError)?;

        fs::write(&file_path, content)
            .map_err(|e| ZackstrapError::WriteFileError(file_path.clone(), e))?;

        println!("  📦 Generated package.json");
        Ok(())
    }

    fn editor_config_to_string(&self, config: &EditorConfig) -> String {
        let mut content = String::new();

        // Root settings
        content.push_str("root = true\n");
        content.push_str(&format!("charset = {}\n", config.charset));
        content.push_str(&format!("end_of_line = {}\n", config.end_of_line));
        content.push_str(&format!("insert_final_newline = {}\n", config.insert_final_newline));
        content.push_str(&format!("trim_trailing_whitespace = {}\n", config.trim_trailing_whitespace));
        content.push_str("\n");

        // Sections
        for (pattern, section) in &config.sections {
            content.push_str(&format!("[{}]\n", pattern));
            content.push_str(&format!("indent_style = {}\n", section.indent_style));
            content.push_str(&format!("indent_size = {}\n", section.indent_size));

            if let Some(ref end_of_line) = section.end_of_line {
                content.push_str(&format!("end_of_line = {}\n", end_of_line));
            }
            if let Some(ref charset) = section.charset {
                content.push_str(&format!("charset = {}\n", charset));
            }
            if let Some(trim) = section.trim_trailing_whitespace {
                content.push_str(&format!("trim_trailing_whitespace = {}\n", trim));
            }
            if let Some(insert) = section.insert_final_newline {
                content.push_str(&format!("insert_final_newline = {}\n", insert));
            }
            content.push_str("\n");
        }

        content
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
"#.to_string()
    }
}
