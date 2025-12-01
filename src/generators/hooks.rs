use crate::error::ZackstrapError;
use std::path::PathBuf;
use tokio::fs;

pub struct GitHooksGenerator {
    target_dir: PathBuf,
}

impl GitHooksGenerator {
    pub fn new(target_dir: PathBuf) -> Self {
        Self { target_dir }
    }

    pub async fn generate_ruby_hooks(
        &self,
        template: &str,
        force: bool,
    ) -> Result<(), ZackstrapError> {
        let hooks_dir = self.target_dir.join(".git").join("hooks");

        // Ensure .git/hooks directory exists
        if !hooks_dir.exists() {
            return Err(ZackstrapError::GitNotInitialized);
        }

        // Generate pre-commit hook
        let pre_commit_content = self.get_ruby_pre_commit_hook(template);
        self.write_hook_file(&hooks_dir.join("pre-commit"), &pre_commit_content, force)
            .await?;

        // Generate pre-push hook
        let pre_push_content = self.get_ruby_pre_push_hook(template);
        self.write_hook_file(&hooks_dir.join("pre-push"), &pre_push_content, force)
            .await?;

        // Generate commit-msg hook
        let commit_msg_content = self.get_commit_msg_hook();
        self.write_hook_file(&hooks_dir.join("commit-msg"), &commit_msg_content, force)
            .await?;

        Ok(())
    }

    pub async fn generate_python_hooks(
        &self,
        template: &str,
        force: bool,
    ) -> Result<(), ZackstrapError> {
        let hooks_dir = self.target_dir.join(".git").join("hooks");

        if !hooks_dir.exists() {
            return Err(ZackstrapError::GitNotInitialized);
        }

        let pre_commit_content = self.get_python_pre_commit_hook(template);
        self.write_hook_file(&hooks_dir.join("pre-commit"), &pre_commit_content, force)
            .await?;

        let pre_push_content = self.get_python_pre_push_hook(template);
        self.write_hook_file(&hooks_dir.join("pre-push"), &pre_push_content, force)
            .await?;

        let commit_msg_content = self.get_commit_msg_hook();
        self.write_hook_file(&hooks_dir.join("commit-msg"), &commit_msg_content, force)
            .await?;

        Ok(())
    }

    pub async fn generate_node_hooks(
        &self,
        template: &str,
        force: bool,
    ) -> Result<(), ZackstrapError> {
        let hooks_dir = self.target_dir.join(".git").join("hooks");

        if !hooks_dir.exists() {
            return Err(ZackstrapError::GitNotInitialized);
        }

        let pre_commit_content = self.get_node_pre_commit_hook(template);
        self.write_hook_file(&hooks_dir.join("pre-commit"), &pre_commit_content, force)
            .await?;

        let pre_push_content = self.get_node_pre_push_hook(template);
        self.write_hook_file(&hooks_dir.join("pre-push"), &pre_push_content, force)
            .await?;

        let commit_msg_content = self.get_commit_msg_hook();
        self.write_hook_file(&hooks_dir.join("commit-msg"), &commit_msg_content, force)
            .await?;

        Ok(())
    }

    pub async fn generate_go_hooks(
        &self,
        template: &str,
        force: bool,
    ) -> Result<(), ZackstrapError> {
        let hooks_dir = self.target_dir.join(".git").join("hooks");

        if !hooks_dir.exists() {
            return Err(ZackstrapError::GitNotInitialized);
        }

        let pre_commit_content = self.get_go_pre_commit_hook(template);
        self.write_hook_file(&hooks_dir.join("pre-commit"), &pre_commit_content, force)
            .await?;

        let pre_push_content = self.get_go_pre_push_hook(template);
        self.write_hook_file(&hooks_dir.join("pre-push"), &pre_push_content, force)
            .await?;

        let commit_msg_content = self.get_commit_msg_hook();
        self.write_hook_file(&hooks_dir.join("commit-msg"), &commit_msg_content, force)
            .await?;

        Ok(())
    }

    pub async fn generate_rust_hooks(
        &self,
        template: &str,
        force: bool,
    ) -> Result<(), ZackstrapError> {
        let hooks_dir = self.target_dir.join(".git").join("hooks");

        if !hooks_dir.exists() {
            return Err(ZackstrapError::GitNotInitialized);
        }

        let pre_commit_content = self.get_rust_pre_commit_hook(template);
        self.write_hook_file(&hooks_dir.join("pre-commit"), &pre_commit_content, force)
            .await?;

        let pre_push_content = self.get_rust_pre_push_hook(template);
        self.write_hook_file(&hooks_dir.join("pre-push"), &pre_push_content, force)
            .await?;

        let commit_msg_content = self.get_commit_msg_hook();
        self.write_hook_file(&hooks_dir.join("commit-msg"), &commit_msg_content, force)
            .await?;

        Ok(())
    }

    pub async fn generate_basic_hooks(&self, force: bool) -> Result<(), ZackstrapError> {
        let hooks_dir = self.target_dir.join(".git").join("hooks");

        if !hooks_dir.exists() {
            return Err(ZackstrapError::GitNotInitialized);
        }

        let pre_commit_content = self.get_basic_pre_commit_hook();
        self.write_hook_file(&hooks_dir.join("pre-commit"), &pre_commit_content, force)
            .await?;

        let pre_push_content = self.get_basic_pre_push_hook();
        self.write_hook_file(&hooks_dir.join("pre-push"), &pre_push_content, force)
            .await?;

        let commit_msg_content = self.get_commit_msg_hook();
        self.write_hook_file(&hooks_dir.join("commit-msg"), &commit_msg_content, force)
            .await?;

        Ok(())
    }

    async fn write_hook_file(
        &self,
        path: &PathBuf,
        content: &str,
        force: bool,
    ) -> Result<(), ZackstrapError> {
        if path.exists() && !force {
            return Err(ZackstrapError::FileAlreadyExists(path.clone()));
        }

        fs::write(path, content).await?;

        // Make the hook executable
        #[cfg(unix)]
        {
            use std::process::Command;
            let _ = Command::new("chmod").arg("+x").arg(path).output();
        }

        Ok(())
    }

    // Ruby hooks
    fn get_ruby_pre_commit_hook(&self, template: &str) -> String {
        match template {
            "rails" => r#"#!/bin/bash
# Ruby Rails Pre-commit Hook
set -e

echo "🔍 Running Ruby Rails pre-commit checks..."

# Check if bundle is available
if ! command -v bundle &> /dev/null; then
    echo "❌ Bundle not found. Please install Ruby and Bundler."
    exit 1
fi

# Install dependencies if needed
if [ ! -d "vendor/bundle" ]; then
    echo "📦 Installing dependencies..."
    bundle install
fi

# Run RuboCop
echo "🔍 Running RuboCop..."
bundle exec rubocop --format simple

# Run Prettier if available
if command -v prettier &> /dev/null; then
    echo "🎨 Running Prettier..."
    prettier --check "**/*.{js,ts,json,md,yml,yaml}" || true
fi

# Run RSpec tests
echo "🧪 Running RSpec tests..."
bundle exec rspec --format documentation

echo "✅ Ruby Rails pre-commit checks passed!"
"#
            .to_string(),
            "sinatra" => r#"#!/bin/bash
# Ruby Sinatra Pre-commit Hook
set -e

echo "🔍 Running Ruby Sinatra pre-commit checks..."

# Check if bundle is available
if ! command -v bundle &> /dev/null; then
    echo "❌ Bundle not found. Please install Ruby and Bundler."
    exit 1
fi

# Install dependencies if needed
if [ ! -d "vendor/bundle" ]; then
    echo "📦 Installing dependencies..."
    bundle install
fi

# Run RuboCop
echo "🔍 Running RuboCop..."
bundle exec rubocop --format simple

# Run Prettier if available
if command -v prettier &> /dev/null; then
    echo "🎨 Running Prettier..."
    prettier --check "**/*.{js,ts,json,md,yml,yaml}" || true
fi

# Run RSpec tests
echo "🧪 Running RSpec tests..."
bundle exec rspec --format documentation

echo "✅ Ruby Sinatra pre-commit checks passed!"
"#
            .to_string(),
            "gem" => r#"#!/bin/bash
# Ruby Gem Pre-commit Hook
set -e

echo "🔍 Running Ruby Gem pre-commit checks..."

# Check if bundle is available
if ! command -v bundle &> /dev/null; then
    echo "❌ Bundle not found. Please install Ruby and Bundler."
    exit 1
fi

# Install dependencies if needed
if [ ! -d "vendor/bundle" ]; then
    echo "📦 Installing dependencies..."
    bundle install
fi

# Run RuboCop
echo "🔍 Running RuboCop..."
bundle exec rubocop --format simple

# Run Prettier if available
if command -v prettier &> /dev/null; then
    echo "🎨 Running Prettier..."
    prettier --check "**/*.{js,ts,json,md,yml,yaml}" || true
fi

# Run RSpec tests
echo "🧪 Running RSpec tests..."
bundle exec rspec --format documentation

# Build gem to check for errors
echo "🔨 Building gem..."
bundle exec gem build *.gemspec

echo "✅ Ruby Gem pre-commit checks passed!"
"#
            .to_string(),
            _ => r#"#!/bin/bash
# Ruby Pre-commit Hook
set -e

echo "🔍 Running Ruby pre-commit checks..."

# Check if bundle is available
if ! command -v bundle &> /dev/null; then
    echo "❌ Bundle not found. Please install Ruby and Bundler."
    exit 1
fi

# Install dependencies if needed
if [ ! -d "vendor/bundle" ]; then
    echo "📦 Installing dependencies..."
    bundle install
fi

# Run RuboCop
echo "🔍 Running RuboCop..."
bundle exec rubocop --format simple

# Run Prettier if available
if command -v prettier &> /dev/null; then
    echo "🎨 Running Prettier..."
    prettier --check "**/*.{js,ts,json,md,yml,yaml}" || true
fi

# Run RSpec tests if available
if [ -f "spec/spec_helper.rb" ] || [ -f "spec/rails_helper.rb" ]; then
    echo "🧪 Running RSpec tests..."
    bundle exec rspec --format documentation
fi

echo "✅ Ruby pre-commit checks passed!"
"#
            .to_string(),
        }
    }

    fn get_ruby_pre_push_hook(&self, template: &str) -> String {
        match template {
            "rails" => r#"#!/bin/bash
# Ruby Rails Pre-push Hook
set -e

echo "🚀 Running Ruby Rails pre-push checks..."

# Run full test suite
echo "🧪 Running full RSpec test suite..."
bundle exec rspec --format progress

# Run security audit
echo "🔒 Running security audit..."
bundle exec bundle audit --update

# Check for outdated dependencies
echo "📦 Checking for outdated dependencies..."
bundle exec bundle outdated || true

echo "✅ Ruby Rails pre-push checks passed!"
"#
            .to_string(),
            _ => r#"#!/bin/bash
# Ruby Pre-push Hook
set -e

echo "🚀 Running Ruby pre-push checks..."

# Run full test suite if available
if [ -f "spec/spec_helper.rb" ] || [ -f "spec/rails_helper.rb" ]; then
    echo "🧪 Running full RSpec test suite..."
    bundle exec rspec --format progress
fi

# Run security audit
echo "🔒 Running security audit..."
bundle exec bundle audit --update || true

# Check for outdated dependencies
echo "📦 Checking for outdated dependencies..."
bundle exec bundle outdated || true

echo "✅ Ruby pre-push checks passed!"
"#
            .to_string(),
        }
    }

    // Python hooks
    fn get_python_pre_commit_hook(&self, template: &str) -> String {
        match template {
            "django" => r#"#!/bin/bash
# Python Django Pre-commit Hook
set -e

echo "🔍 Running Python Django pre-commit checks..."

# Check if Python is available
if ! command -v python &> /dev/null; then
    echo "❌ Python not found. Please install Python."
    exit 1
fi

# Activate virtual environment if it exists
if [ -f "venv/bin/activate" ]; then
    echo "🐍 Activating virtual environment..."
    source venv/bin/activate
elif [ -f ".venv/bin/activate" ]; then
    echo "🐍 Activating virtual environment..."
    source .venv/bin/activate
fi

# Install dependencies if needed
if [ -f "requirements.txt" ]; then
    echo "📦 Installing dependencies..."
    pip install -r requirements.txt
fi

if [ -f "requirements-dev.txt" ]; then
    echo "📦 Installing dev dependencies..."
    pip install -r requirements-dev.txt
fi

# Run Black formatting check
echo "🎨 Running Black formatting check..."
black --check .

# Run Flake8 linting
echo "🔍 Running Flake8 linting..."
flake8 .

# Run MyPy type checking
echo "🔬 Running MyPy type checking..."
mypy .

# Run Django tests
echo "🧪 Running Django tests..."
python manage.py test

echo "✅ Python Django pre-commit checks passed!"
"#
            .to_string(),
            "flask" => r#"#!/bin/bash
# Python Flask Pre-commit Hook
set -e

echo "🔍 Running Python Flask pre-commit checks..."

# Check if Python is available
if ! command -v python &> /dev/null; then
    echo "❌ Python not found. Please install Python."
    exit 1
fi

# Activate virtual environment if it exists
if [ -f "venv/bin/activate" ]; then
    echo "🐍 Activating virtual environment..."
    source venv/bin/activate
elif [ -f ".venv/bin/activate" ]; then
    echo "🐍 Activating virtual environment..."
    source .venv/bin/activate
fi

# Install dependencies if needed
if [ -f "requirements.txt" ]; then
    echo "📦 Installing dependencies..."
    pip install -r requirements.txt
fi

if [ -f "requirements-dev.txt" ]; then
    echo "📦 Installing dev dependencies..."
    pip install -r requirements-dev.txt
fi

# Run Black formatting check
echo "🎨 Running Black formatting check..."
black --check .

# Run Flake8 linting
echo "🔍 Running Flake8 linting..."
flake8 .

# Run MyPy type checking
echo "🔬 Running MyPy type checking..."
mypy .

# Run Pytest tests
echo "🧪 Running Pytest tests..."
pytest

echo "✅ Python Flask pre-commit checks passed!"
"#
            .to_string(),
            _ => r#"#!/bin/bash
# Python Pre-commit Hook
set -e

echo "🔍 Running Python pre-commit checks..."

# Check if Python is available
if ! command -v python &> /dev/null; then
    echo "❌ Python not found. Please install Python."
    exit 1
fi

# Activate virtual environment if it exists
if [ -f "venv/bin/activate" ]; then
    echo "🐍 Activating virtual environment..."
    source venv/bin/activate
elif [ -f ".venv/bin/activate" ]; then
    echo "🐍 Activating virtual environment..."
    source .venv/bin/activate
fi

# Install dependencies if needed
if [ -f "requirements.txt" ]; then
    echo "📦 Installing dependencies..."
    pip install -r requirements.txt
fi

if [ -f "requirements-dev.txt" ]; then
    echo "📦 Installing dev dependencies..."
    pip install -r requirements-dev.txt
fi

# Run Black formatting check
echo "🎨 Running Black formatting check..."
black --check .

# Run Flake8 linting
echo "🔍 Running Flake8 linting..."
flake8 .

# Run MyPy type checking
echo "🔬 Running MyPy type checking..."
mypy .

# Run Pytest tests if available
if [ -f "pytest.ini" ] || [ -f "pyproject.toml" ] || [ -d "tests" ]; then
    echo "🧪 Running Pytest tests..."
    pytest
fi

echo "✅ Python pre-commit checks passed!"
"#
            .to_string(),
        }
    }

    fn get_python_pre_push_hook(&self, template: &str) -> String {
        match template {
            "django" => r#"#!/bin/bash
# Python Django Pre-push Hook
set -e

echo "🚀 Running Python Django pre-push checks..."

# Activate virtual environment if it exists
if [ -f "venv/bin/activate" ]; then
    echo "🐍 Activating virtual environment..."
    source venv/bin/activate
elif [ -f ".venv/bin/activate" ]; then
    echo "🐍 Activating virtual environment..."
    source .venv/bin/activate
fi

# Run full test suite
echo "🧪 Running full Django test suite..."
python manage.py test --parallel

# Run security check
echo "🔒 Running Django security check..."
python manage.py check --deploy

# Run coverage report
echo "📊 Running coverage report..."
coverage run --source='.' manage.py test
coverage report

echo "✅ Python Django pre-push checks passed!"
"#
            .to_string(),
            _ => r#"#!/bin/bash
# Python Pre-push Hook
set -e

echo "🚀 Running Python pre-push checks..."

# Activate virtual environment if it exists
if [ -f "venv/bin/activate" ]; then
    echo "🐍 Activating virtual environment..."
    source venv/bin/activate
elif [ -f ".venv/bin/activate" ]; then
    echo "🐍 Activating virtual environment..."
    source .venv/bin/activate
fi

# Run full test suite
echo "🧪 Running full Pytest test suite..."
pytest --verbose

# Run coverage report
echo "📊 Running coverage report..."
coverage run -m pytest
coverage report

echo "✅ Python pre-push checks passed!"
"#
            .to_string(),
        }
    }

    // Node.js hooks
    fn get_node_pre_commit_hook(&self, template: &str) -> String {
        match template {
            "express" => r#"#!/bin/bash
# Node.js Express Pre-commit Hook
set -e

echo "🔍 Running Node.js Express pre-commit checks..."

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js not found. Please install Node.js."
    exit 1
fi

# Check if npm is available
if ! command -v npm &> /dev/null; then
    echo "❌ npm not found. Please install npm."
    exit 1
fi

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
fi

# Run ESLint
echo "🔍 Running ESLint..."
npm run lint

# Run Prettier
echo "🎨 Running Prettier..."
npm run format

# Run TypeScript check if available
if [ -f "tsconfig.json" ]; then
    echo "🔬 Running TypeScript check..."
    npx tsc --noEmit
fi

# Run Jest tests
echo "🧪 Running Jest tests..."
npm test

echo "✅ Node.js Express pre-commit checks passed!"
"#
            .to_string(),
            "react" => r#"#!/bin/bash
# Node.js React Pre-commit Hook
set -e

echo "🔍 Running Node.js React pre-commit checks..."

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js not found. Please install Node.js."
    exit 1
fi

# Check if npm is available
if ! command -v npm &> /dev/null; then
    echo "❌ npm not found. Please install npm."
    exit 1
fi

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
fi

# Run ESLint
echo "🔍 Running ESLint..."
npm run lint

# Run Prettier
echo "🎨 Running Prettier..."
npm run format

# Run TypeScript check if available
if [ -f "tsconfig.json" ]; then
    echo "🔬 Running TypeScript check..."
    npx tsc --noEmit
fi

# Run React tests
echo "🧪 Running React tests..."
npm test -- --watchAll=false

echo "✅ Node.js React pre-commit checks passed!"
"#
            .to_string(),
            _ => r#"#!/bin/bash
# Node.js Pre-commit Hook
set -e

echo "🔍 Running Node.js pre-commit checks..."

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js not found. Please install Node.js."
    exit 1
fi

# Check if npm is available
if ! command -v npm &> /dev/null; then
    echo "❌ npm not found. Please install npm."
    exit 1
fi

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
fi

# Run ESLint if available
if [ -f ".eslintrc.js" ] || [ -f ".eslintrc.json" ]; then
    echo "🔍 Running ESLint..."
    npm run lint
fi

# Run Prettier if available
if [ -f ".prettierrc" ] || [ -f ".prettierrc.js" ]; then
    echo "🎨 Running Prettier..."
    npm run format
fi

# Run TypeScript check if available
if [ -f "tsconfig.json" ]; then
    echo "🔬 Running TypeScript check..."
    npx tsc --noEmit
fi

# Run tests if available
if [ -f "package.json" ] && grep -q '"test"' package.json; then
    echo "🧪 Running tests..."
    npm test
fi

echo "✅ Node.js pre-commit checks passed!"
"#
            .to_string(),
        }
    }

    fn get_node_pre_push_hook(&self, template: &str) -> String {
        match template {
            "express" => r#"#!/bin/bash
# Node.js Express Pre-push Hook
set -e

echo "🚀 Running Node.js Express pre-push checks..."

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
fi

# Run full test suite
echo "🧪 Running full Jest test suite..."
npm test -- --coverage

# Run security audit
echo "🔒 Running security audit..."
npm audit

# Build project
echo "🔨 Building project..."
npm run build

echo "✅ Node.js Express pre-push checks passed!"
"#
            .to_string(),
            "react" => r#"#!/bin/bash
# Node.js React Pre-push Hook
set -e

echo "🚀 Running Node.js React pre-push checks..."

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
fi

# Run full test suite
echo "🧪 Running full React test suite..."
npm test -- --coverage --watchAll=false

# Run security audit
echo "🔒 Running security audit..."
npm audit

# Build project
echo "🔨 Building project..."
npm run build

echo "✅ Node.js React pre-push checks passed!"
"#
            .to_string(),
            _ => r#"#!/bin/bash
# Node.js Pre-push Hook
set -e

echo "🚀 Running Node.js pre-push checks..."

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
fi

# Run full test suite if available
if [ -f "package.json" ] && grep -q '"test"' package.json; then
    echo "🧪 Running full test suite..."
    npm test -- --coverage
fi

# Run security audit
echo "🔒 Running security audit..."
npm audit

# Build project if available
if [ -f "package.json" ] && grep -q '"build"' package.json; then
    echo "🔨 Building project..."
    npm run build
fi

echo "✅ Node.js pre-push checks passed!"
"#
            .to_string(),
        }
    }

    // Go hooks
    fn get_go_pre_commit_hook(&self, template: &str) -> String {
        match template {
            "web" => r#"#!/bin/bash
# Go Web Pre-commit Hook
set -e

echo "🔍 Running Go web pre-commit checks..."

# Check if Go is available
if ! command -v go &> /dev/null; then
    echo "❌ Go not found. Please install Go."
    exit 1
fi

# Download dependencies
echo "📦 Downloading dependencies..."
go mod download
go mod tidy

# Run go fmt
echo "🎨 Running go fmt..."
go fmt ./...

# Run go vet
echo "🔍 Running go vet..."
go vet ./...

# Run golangci-lint if available
if command -v golangci-lint &> /dev/null; then
    echo "🔍 Running golangci-lint..."
    golangci-lint run
fi

# Run tests
echo "🧪 Running Go tests..."
go test ./...

echo "✅ Go web pre-commit checks passed!"
"#
            .to_string(),
            "cli" => r#"#!/bin/bash
# Go CLI Pre-commit Hook
set -e

echo "🔍 Running Go CLI pre-commit checks..."

# Check if Go is available
if ! command -v go &> /dev/null; then
    echo "❌ Go not found. Please install Go."
    exit 1
fi

# Download dependencies
echo "📦 Downloading dependencies..."
go mod download
go mod tidy

# Run go fmt
echo "🎨 Running go fmt..."
go fmt ./...

# Run go vet
echo "🔍 Running go vet..."
go vet ./...

# Run golangci-lint if available
if command -v golangci-lint &> /dev/null; then
    echo "🔍 Running golangci-lint..."
    golangci-lint run
fi

# Run tests
echo "🧪 Running Go tests..."
go test ./...

# Build CLI to check for errors
echo "🔨 Building CLI..."
go build -o /tmp/cli ./cmd/cli

echo "✅ Go CLI pre-commit checks passed!"
"#
            .to_string(),
            _ => r#"#!/bin/bash
# Go Pre-commit Hook
set -e

echo "🔍 Running Go pre-commit checks..."

# Check if Go is available
if ! command -v go &> /dev/null; then
    echo "❌ Go not found. Please install Go."
    exit 1
fi

# Download dependencies
echo "📦 Downloading dependencies..."
go mod download
go mod tidy

# Run go fmt
echo "🎨 Running go fmt..."
go fmt ./...

# Run go vet
echo "🔍 Running go vet..."
go vet ./...

# Run golangci-lint if available
if command -v golangci-lint &> /dev/null; then
    echo "🔍 Running golangci-lint..."
    golangci-lint run
fi

# Run tests
echo "🧪 Running Go tests..."
go test ./...

echo "✅ Go pre-commit checks passed!"
"#
            .to_string(),
        }
    }

    fn get_go_pre_push_hook(&self, template: &str) -> String {
        match template {
            "web" => r#"#!/bin/bash
# Go Web Pre-push Hook
set -e

echo "🚀 Running Go web pre-push checks..."

# Download dependencies
echo "📦 Downloading dependencies..."
go mod download
go mod tidy

# Run full test suite with coverage
echo "🧪 Running full Go test suite with coverage..."
go test -coverprofile=coverage.out ./...
go tool cover -func=coverage.out

# Run golangci-lint with all checks
if command -v golangci-lint &> /dev/null; then
    echo "🔍 Running golangci-lint with all checks..."
    golangci-lint run --enable-all
fi

# Build for multiple platforms
echo "🔨 Building for multiple platforms..."
GOOS=linux GOARCH=amd64 go build -o /tmp/app-linux-amd64 .
GOOS=darwin GOARCH=amd64 go build -o /tmp/app-darwin-amd64 .
GOOS=windows GOARCH=amd64 go build -o /tmp/app-windows-amd64.exe .

echo "✅ Go web pre-push checks passed!"
"#
            .to_string(),
            _ => r#"#!/bin/bash
# Go Pre-push Hook
set -e

echo "🚀 Running Go pre-push checks..."

# Download dependencies
echo "📦 Downloading dependencies..."
go mod download
go mod tidy

# Run full test suite with coverage
echo "🧪 Running full Go test suite with coverage..."
go test -coverprofile=coverage.out ./...
go tool cover -func=coverage.out

# Run golangci-lint with all checks
if command -v golangci-lint &> /dev/null; then
    echo "🔍 Running golangci-lint with all checks..."
    golangci-lint run --enable-all
fi

echo "✅ Go pre-push checks passed!"
"#
            .to_string(),
        }
    }

    // Rust hooks
    fn get_rust_pre_commit_hook(&self, template: &str) -> String {
        match template {
            "web" => r#"#!/bin/bash
# Rust Web Pre-commit Hook
set -e

echo "🔍 Running Rust web pre-commit checks..."

# Check if Rust is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust."
    exit 1
fi

# Check code
echo "🔍 Running cargo check..."
cargo check

# Run clippy
echo "🔍 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Format code
echo "🎨 Running rustfmt..."
cargo fmt --all -- --check

# Run tests
echo "🧪 Running Rust tests..."
cargo test

echo "✅ Rust web pre-commit checks passed!"
"#
            .to_string(),
            "cli" => r#"#!/bin/bash
# Rust CLI Pre-commit Hook
set -e

echo "🔍 Running Rust CLI pre-commit checks..."

# Check if Rust is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust."
    exit 1
fi

# Check code
echo "🔍 Running cargo check..."
cargo check

# Run clippy
echo "🔍 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Format code
echo "🎨 Running rustfmt..."
cargo fmt --all -- --check

# Run tests
echo "🧪 Running Rust tests..."
cargo test

# Build CLI to check for errors
echo "🔨 Building CLI..."
cargo build

echo "✅ Rust CLI pre-commit checks passed!"
"#
            .to_string(),
            _ => r#"#!/bin/bash
# Rust Pre-commit Hook
set -e

echo "🔍 Running Rust pre-commit checks..."

# Check if Rust is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust."
    exit 1
fi

# Check code
echo "🔍 Running cargo check..."
cargo check

# Run clippy
echo "🔍 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Format code
echo "🎨 Running rustfmt..."
cargo fmt --all -- --check

# Run tests
echo "🧪 Running Rust tests..."
cargo test

echo "✅ Rust pre-commit checks passed!"
"#
            .to_string(),
        }
    }

    fn get_rust_pre_push_hook(&self, template: &str) -> String {
        match template {
            "web" => r#"#!/bin/bash
# Rust Web Pre-push Hook
set -e

echo "🚀 Running Rust web pre-push checks..."

# Run full test suite
echo "🧪 Running full Rust test suite..."
cargo test --all-features

# Run clippy with all checks
echo "🔍 Running clippy with all checks..."
cargo clippy --all-targets --all-features -- -D warnings

# Run tests with coverage if available
if command -v cargo-llvm-cov &> /dev/null; then
    echo "📊 Running tests with coverage..."
    cargo llvm-cov --html
fi

# Build for release
echo "🔨 Building for release..."
cargo build --release

echo "✅ Rust web pre-push checks passed!"
"#
            .to_string(),
            _ => r#"#!/bin/bash
# Rust Pre-push Hook
set -e

echo "🚀 Running Rust pre-push checks..."

# Run full test suite
echo "🧪 Running full Rust test suite..."
cargo test --all-features

# Run clippy with all checks
echo "🔍 Running clippy with all checks..."
cargo clippy --all-targets --all-features -- -D warnings

# Run tests with coverage if available
if command -v cargo-llvm-cov &> /dev/null; then
    echo "📊 Running tests with coverage..."
    cargo llvm-cov --html
fi

# Build for release
echo "🔨 Building for release..."
cargo build --release

echo "✅ Rust pre-push checks passed!"
"#
            .to_string(),
        }
    }

    // Basic hooks
    fn get_basic_pre_commit_hook(&self) -> String {
        r#"#!/bin/bash
# Basic Pre-commit Hook
set -e

echo "🔍 Running basic pre-commit checks..."

# Run Prettier if available
if command -v prettier &> /dev/null; then
    echo "🎨 Running Prettier..."
    prettier --check "**/*.{js,ts,json,md,yml,yaml}" || true
fi

# Run any available linters
if [ -f ".eslintrc.js" ] || [ -f ".eslintrc.json" ]; then
    echo "🔍 Running ESLint..."
    npx eslint . || true
fi

if [ -f ".rubocop.yml" ]; then
    echo "🔍 Running RuboCop..."
    bundle exec rubocop || true
fi

if [ -f "pyproject.toml" ]; then
    echo "🔍 Running Black..."
    black --check . || true
fi

echo "✅ Basic pre-commit checks passed!"
"#
        .to_string()
    }

    fn get_basic_pre_push_hook(&self) -> String {
        r#"#!/bin/bash
# Basic Pre-push Hook
set -e

echo "🚀 Running basic pre-push checks..."

# Run any available tests
if [ -f "package.json" ] && grep -q '"test"' package.json; then
    echo "🧪 Running npm tests..."
    npm test || true
fi

if [ -f "Gemfile" ]; then
    echo "🧪 Running RSpec tests..."
    bundle exec rspec || true
fi

if [ -f "pytest.ini" ] || [ -f "pyproject.toml" ]; then
    echo "🧪 Running Pytest tests..."
    pytest || true
fi

if [ -f "Cargo.toml" ]; then
    echo "🧪 Running Cargo tests..."
    cargo test || true
fi

if [ -f "go.mod" ]; then
    echo "🧪 Running Go tests..."
    go test ./... || true
fi

echo "✅ Basic pre-push checks passed!"
"#
        .to_string()
    }

    // Common hooks
    fn get_commit_msg_hook(&self) -> String {
        r#"#!/bin/bash
# Commit Message Hook
set -e

# Get the commit message
commit_msg_file=$1
commit_msg=$(cat "$commit_msg_file")

# Check if commit message is empty
if [ -z "$commit_msg" ]; then
    echo "❌ Commit message cannot be empty"
    exit 1
fi

# Check if commit message starts with a type
if ! echo "$commit_msg" | grep -qE "^(feat|fix|docs|style|refactor|test|chore|perf|ci|build|revert)(\(.+\))?: .+"; then
    echo "❌ Commit message must follow conventional commits format:"
    echo "   <type>(<scope>): <description>"
    echo "   Examples:"
    echo "   feat: add new feature"
    echo "   fix(api): resolve authentication bug"
    echo "   docs: update README"
    exit 1
fi

# Check minimum length
if [ ${#commit_msg} -lt 10 ]; then
    echo "❌ Commit message must be at least 10 characters long"
    exit 1
fi

echo "✅ Commit message is valid"
"#.to_string()
    }
}
