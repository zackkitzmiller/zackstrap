use crate::config::PackageJson;
use crate::error::ZackstrapError;
use super::common::FileGenerator;

impl super::ConfigGenerator {
    #[allow(dead_code)]
    pub async fn generate_node(&self, force: bool) -> Result<(), ZackstrapError> {
        self.generate_node_with_template(force, "default").await
    }

    pub async fn generate_node_with_template(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        // Generate basic configs first
        self.generate_basic_with_template(force, false, template).await?;

        // Generate Node.js-specific configs
        self.generate_nvmrc(force).await?;
        self.generate_eslint_config(force, template).await?;
        self.generate_node_package_json(force, template).await?;

        // Overwrite the basic justfile with Node.js-specific one
        self.generate_node_justfile(true, template).await?;

        Ok(())
    }

    pub async fn dry_run_node_with_template(&self, template: &str) -> Result<(), ZackstrapError> {
        println!("  📝 Would generate .editorconfig");
        println!("  🎨 Would generate .prettierrc (template: {})", template);
        println!("  🔧 Would generate justfile");
        println!("  🟢 Would generate .nvmrc (20)");
        println!("  🔍 Would generate .eslintrc.js (template: {})", template);
        println!("  📦 Would generate package.json (template: {})", template);
        println!("  🔧 Would generate Node.js justfile (template: {})", template);
        Ok(())
    }

    async fn generate_nvmrc(&self, force: bool) -> Result<(), ZackstrapError> {
        let content = "20\n";
        self.write_file_if_not_exists(".nvmrc", content, force, false).await
    }

    async fn generate_eslint_config(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let content = match template {
            "express" => r#"{
  "env": {
    "browser": true,
    "node": true,
    "es2021": true
  },
  "extends": ["eslint:recommended"],
  "parserOptions": {
    "ecmaVersion": 12,
    "sourceType": "module"
  },
  "rules": {}
}
"#,
            "react" => r#"{
  "env": {
    "browser": true,
    "node": true,
    "es2021": true
  },
  "extends": ["eslint:recommended"],
  "parserOptions": {
    "ecmaVersion": 12,
    "sourceType": "module"
  },
  "rules": {}
}
"#,
            _ => r#"{
  "env": {
    "browser": true,
    "node": true,
    "es2021": true
  },
  "extends": ["eslint:recommended"],
  "parserOptions": {
    "ecmaVersion": 12,
    "sourceType": "module"
  },
  "rules": {}
}
"#,
        };
        self.write_file_if_not_exists(".eslintrc.json", content, force, false).await
    }

    async fn generate_node_package_json(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let package_json = PackageJson::from_template(template);
        let content = package_json.to_string();
        self.write_file_if_not_exists("package.json", &content, force, false).await
    }

    async fn generate_node_justfile(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let content = match template {
            "express" => r#"# Express.js project justfile
default:
    @echo "Available Express.js commands:"
    @just --list

# Start development server
dev:
    @echo "Starting Express.js development server..."
    @npm run dev

# Start production server
start:
    @echo "Starting Express.js production server..."
    @npm start

# Run tests
test:
    @echo "Running Express.js tests..."
    @npm test

# Run linting
lint:
    @echo "Running ESLint..."
    @npm run lint

# Install dependencies
install:
    @echo "Installing Node.js dependencies..."
    @npm install

# Build project
build:
    @echo "Building Express.js project..."
    @npm run build
"#,
            "react" => r#"# React project justfile
default:
    @echo "Available React commands:"
    @just --list

# Start development server
dev:
    @echo "Starting React development server..."
    @npm start

# Build for production
build:
    @echo "Building React project for production..."
    @npm run build

# Run tests
test:
    @echo "Running React tests..."
    @npm test

# Run linting
lint:
    @echo "Running ESLint..."
    @npm run lint

# Install dependencies
install:
    @echo "Installing Node.js dependencies..."
    @npm install

# Eject (use with caution)
eject:
    @echo "Ejecting React app..."
    @npm run eject
"#,
            _ => r#"# Node.js project justfile
default:
    @echo "Available Node.js commands:"
    @just --list

# Start development server
dev:
    @echo "Starting Node.js development server..."
    @npm run dev

# Start production server
start:
    @echo "Starting Node.js production server..."
    @npm start

# Run tests
test:
    @echo "Running Node.js tests..."
    @npm test

# Run linting
lint:
    @echo "Running ESLint..."
    @npm run lint

# Install dependencies
install:
    @echo "Installing Node.js dependencies..."
    @npm install

# Build project
build:
    @echo "Building Node.js project..."
    @npm run build
"#,
        };
        self.write_file_if_not_exists("justfile", content, force, false).await
    }
}
