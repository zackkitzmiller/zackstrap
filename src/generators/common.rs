use crate::error::ZackstrapError;
use std::fs;
use std::path::PathBuf;

pub trait FileGenerator {
    fn target_dir(&self) -> &PathBuf;

    async fn write_file_if_not_exists(
        &self,
        filename: &str,
        content: &str,
        force: bool,
    ) -> Result<(), ZackstrapError> {
        let file_path = self.target_dir().join(filename);

        if file_path.exists() && !force {
            return Err(ZackstrapError::FileExists(file_path));
        }

        // Ensure parent directory exists
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&file_path, content)?;
        Ok(())
    }
}

impl FileGenerator for super::ConfigGenerator {
    fn target_dir(&self) -> &PathBuf {
        &self.target_dir
    }
}
