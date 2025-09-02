use super::common::FileGenerator;
use crate::error::ZackstrapError;

impl super::ConfigGenerator {
    #[allow(dead_code)]
    pub async fn generate_python(&self, force: bool) -> Result<(), ZackstrapError> {
        self.generate_python_with_template(force, "default").await
    }

    pub async fn generate_python_with_template(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        // Generate basic configs first
        self.generate_basic_with_template(force, false, template)
            .await?;

        // Generate Python-specific configs
        self.generate_python_version(force).await?;
        self.generate_pyproject_toml(force, template).await?;
        self.generate_flake8_config(force).await?;
        self.generate_requirements_dev(force).await?;

        // Overwrite the basic justfile with Python-specific one
        self.generate_python_justfile(true, template).await?;

        Ok(())
    }

    pub async fn dry_run_python_with_template(&self, template: &str) -> Result<(), ZackstrapError> {
        println!("  📝 Would generate .editorconfig");
        println!("  🎨 Would generate .prettierrc (template: {})", template);
        println!("  🔧 Would generate justfile");
        println!("  🐍 Would generate .python-version (3.12)");
        println!(
            "  📋 Would generate pyproject.toml (template: {})",
            template
        );
        println!("  🔍 Would generate .flake8");
        println!("  📦 Would generate requirements-dev.txt");
        println!(
            "  🔧 Would generate Python justfile (template: {})",
            template
        );
        Ok(())
    }

    async fn generate_python_version(&self, force: bool) -> Result<(), ZackstrapError> {
        let content = "3.12\n";
        self.write_file_if_not_exists(".python-version", content, force, false)
            .await
    }

    async fn generate_pyproject_toml(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let content = match template {
            "django" => {
                r#"[build-system]
requires = ["setuptools>=61.0", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "django-project"
version = "0.1.0"
description = "A Django web application"
authors = [{name = "Developer", email = "dev@example.com"}]
readme = "README.md"
requires-python = ">=3.12"
dependencies = [
    "django>=4.2",
    "djangorestframework>=3.14",
]

[project.optional-dependencies]
dev = [
    "pytest>=7.0",
    "pytest-django>=4.5",
    "black>=23.0",
    "flake8>=6.0",
    "mypy>=1.0",
]

[tool.black]
line-length = 88
target-version = ['py312']

[tool.flake8]
max-line-length = 88
extend-ignore = ["E203", "W503"]

[tool.mypy]
python_version = "3.12"
warn_return_any = true
warn_unused_configs = true
strict = true

[tool.django-stubs]
django_settings_module = "project.settings"
"#
            }
            "flask" => {
                r#"[build-system]
requires = ["setuptools>=61.0", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "flask-project"
version = "0.1.0"
description = "A Flask web application"
authors = [{name = "Developer", email = "dev@example.com"}]
readme = "README.md"
requires-python = ">=3.12"
dependencies = [
    "flask>=3.0",
    "flask-sqlalchemy>=3.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=7.0",
    "pytest-flask>=1.2",
    "black>=23.0",
    "flake8>=6.0",
    "mypy>=1.0",
]

[tool.black]
line-length = 88
target-version = ['py312']

[tool.flake8]
max-line-length = 88
extend-ignore = ["E203", "W503"]

[tool.mypy]
python_version = "3.12"
warn_return_any = true
warn_unused_configs = true
strict = true

[tool.flask]
app_name = "app"
"#
            }
            _ => {
                r#"[build-system]
requires = ["setuptools>=61.0", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "python-project"
version = "0.1.0"
description = "A Python project"
authors = [{name = "Developer", email = "dev@example.com"}]
readme = "README.md"
requires-python = ">=3.12"
dependencies = []

[project.optional-dependencies]
dev = [
    "pytest>=7.0",
    "black>=23.0",
    "flake8>=6.0",
    "mypy>=1.0",
]

[tool.black]
line-length = 88
target-version = ['py312']

[tool.flake8]
max-line-length = 88
extend-ignore = ["E203", "W503"]

[tool.mypy]
python_version = "3.12"
warn_return_any = true
warn_unused_configs = true
strict = true
"#
            }
        };
        self.write_file_if_not_exists("pyproject.toml", content, force, false)
            .await
    }

    async fn generate_flake8_config(&self, force: bool) -> Result<(), ZackstrapError> {
        let content = r#"[flake8]
max-line-length = 88
extend-ignore = E203, W503
exclude = .git,__pycache__,build,dist,.venv,venv
"#;
        self.write_file_if_not_exists(".flake8", content, force, false)
            .await
    }

    async fn generate_requirements_dev(&self, force: bool) -> Result<(), ZackstrapError> {
        let content = r#"# Development dependencies
pytest==7.4.4
black==23.12.1
flake8==6.1.0
mypy==1.8.0
pytest-cov==4.1.0
"#;
        self.write_file_if_not_exists("requirements-dev.txt", content, force, false)
            .await
    }

    async fn generate_python_justfile(
        &self,
        force: bool,
        template: &str,
    ) -> Result<(), ZackstrapError> {
        let content = match template {
            "django" => {
                r#"# Django project justfile
default:
    @echo "Available Django commands:"
    @just --list

# Start Django development server
server:
    @echo "Starting Django development server..."
    @python manage.py runserver

# Run Django tests
test:
    @echo "Running Django tests..."
    @python manage.py test

# Run migrations
migrate:
    @echo "Running Django migrations..."
    @python manage.py migrate

# Create superuser
createsuperuser:
    @echo "Creating Django superuser..."
    @python manage.py createsuperuser

# Collect static files
collectstatic:
    @echo "Collecting static files..."
    @python manage.py collectstatic

# Install dependencies
install:
    @echo "Installing Python dependencies..."
    @pip install -r requirements.txt
    @pip install -r requirements-dev.txt
"#
            }
            "flask" => {
                r#"# Flask project justfile
default:
    @echo "Available Flask commands:"
    @just --list

# Start Flask development server
server:
    @echo "Starting Flask development server..."
    @python app.py

# Run Flask tests
test:
    @echo "Running Flask tests..."
    @pytest

# Install dependencies
install:
    @echo "Installing Python dependencies..."
    @pip install -r requirements.txt
    @pip install -r requirements-dev.txt
"#
            }
            _ => {
                r#"# Python project justfile
default:
    @echo "Available Python commands:"
    @just --list

# Run tests
test:
    @echo "Running Python tests..."
    @pytest

# Format code
format:
    @echo "Formatting Python code..."
    @black .

# Lint code
lint:
    @echo "Linting Python code..."
    @flake8 .

# Type check
typecheck:
    @echo "Type checking Python code..."
    @mypy .

# Install dependencies
install:
    @echo "Installing Python dependencies..."
    @pip install -r requirements.txt
    @pip install -r requirements-dev.txt
"#
            }
        };
        self.write_file_if_not_exists("justfile", content, force, false)
            .await
    }
}
