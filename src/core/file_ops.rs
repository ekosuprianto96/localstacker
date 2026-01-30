use std::fs;
use std::path::Path;

use crate::core::FileOperations;
use crate::error::{Error, Result};
use crate::utils::log_verbose;

pub struct FileOps;

impl FileOps {
    pub fn new() -> Self {
        FileOps
    }
}

impl FileOperations for FileOps {
    fn ensure_directory(&self, path: &str) -> Result<()> {
        log_verbose(&format!("Ensuring directory exists: {}", path));

        if !Path::new(path).exists() {
            fs::create_dir_all(path)
                .map_err(|e| Error::Io(e))?;
        }

        Ok(())
    }

    fn copy_file(&self, from: &str, to: &str) -> Result<()> {
        log_verbose(&format!("Copying {} -> {}", from, to));

        fs::copy(from, to)
            .map_err(|e| Error::Io(e))?;

        Ok(())
    }

    fn remove_file(&self, path: &str) -> Result<()> {
        log_verbose(&format!("Removing file: {}", path));

        if Path::new(path).exists() {
            fs::remove_file(path)
                .map_err(|e| Error::Io(e))?;
        }

        Ok(())
    }

    fn file_exists(&self, path: &str) -> bool {
        Path::new(path).exists()
    }
}