use anyhow::{Context, Result};
use glob::glob;
use std::fs;
use std::path::{Path, PathBuf};

pub struct FileHandler {
    backup_enabled: bool,
}

impl FileHandler {
    pub fn new(backup_enabled: bool) -> Self {
        Self { backup_enabled }
    }

    pub fn find_typescript_files(&self, paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        for path in paths {
            if path.is_file() {
                if self.is_typescript_file(path) {
                    files.push(path.clone());
                }
            } else if path.is_dir() {
                self.find_ts_files_in_dir(path, &mut files)?;
            } else {
                // Treat as glob pattern
                let pattern = path.to_str().context("Invalid path")?;
                for entry in glob(pattern).context("Failed to read glob pattern")? {
                    let file = entry.context("Failed to process glob entry")?;
                    if self.is_typescript_file(&file) {
                        files.push(file);
                    }
                }
            }
        }

        Ok(files)
    }

    fn find_ts_files_in_dir(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        for entry in fs::read_dir(dir).context("Failed to read directory")? {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();
            
            if path.is_dir() {
                // Skip node_modules and hidden directories
                if let Some(name) = path.file_name() {
                    let name_str = name.to_string_lossy();
                    if name_str != "node_modules" && !name_str.starts_with('.') {
                        self.find_ts_files_in_dir(&path, files)?;
                    }
                }
            } else if self.is_typescript_file(&path) {
                files.push(path);
            }
        }
        Ok(())
    }

    fn is_typescript_file(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| matches!(ext, "ts" | "tsx" | "mts" | "cts"))
            .unwrap_or(false)
    }

    pub fn read_file(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))
    }

    pub fn write_file(&self, path: &Path, content: &str) -> Result<()> {
        if self.backup_enabled {
            self.create_backup(path)?;
        }

        fs::write(path, content)
            .with_context(|| format!("Failed to write file: {}", path.display()))
    }

    fn create_backup(&self, path: &Path) -> Result<()> {
        let backup_path = path.with_extension(format!(
            "{}.bak",
            path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
        ));

        fs::copy(path, &backup_path)
            .with_context(|| format!("Failed to create backup: {}", backup_path.display()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_is_typescript_file() {
        let handler = FileHandler::new(false);
        
        assert!(handler.is_typescript_file(Path::new("test.ts")));
        assert!(handler.is_typescript_file(Path::new("test.tsx")));
        assert!(handler.is_typescript_file(Path::new("test.mts")));
        assert!(handler.is_typescript_file(Path::new("test.cts")));
        
        assert!(!handler.is_typescript_file(Path::new("test.js")));
        assert!(!handler.is_typescript_file(Path::new("test.jsx")));
        assert!(!handler.is_typescript_file(Path::new("test.txt")));
        assert!(!handler.is_typescript_file(Path::new("test")));
    }

    #[test]
    fn test_find_typescript_files_single_file() {
        let temp_dir = TempDir::new().unwrap();
        let ts_file = temp_dir.path().join("test.ts");
        fs::write(&ts_file, "// test").unwrap();
        
        let handler = FileHandler::new(false);
        let files = handler.find_typescript_files(&[ts_file.clone()]).unwrap();
        
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], ts_file);
    }

    #[test]
    fn test_find_typescript_files_in_directory() {
        let temp_dir = TempDir::new().unwrap();
        let ts_file1 = temp_dir.path().join("file1.ts");
        let ts_file2 = temp_dir.path().join("file2.tsx");
        let js_file = temp_dir.path().join("file3.js");
        
        fs::write(&ts_file1, "// test1").unwrap();
        fs::write(&ts_file2, "// test2").unwrap();
        fs::write(&js_file, "// test3").unwrap();
        
        let handler = FileHandler::new(false);
        let mut files = handler.find_typescript_files(&[temp_dir.path().to_path_buf()]).unwrap();
        files.sort();
        
        assert_eq!(files.len(), 2);
        assert!(files.contains(&ts_file1));
        assert!(files.contains(&ts_file2));
        assert!(!files.contains(&js_file));
    }

    #[test]
    fn test_skip_node_modules() {
        let temp_dir = TempDir::new().unwrap();
        let node_modules = temp_dir.path().join("node_modules");
        fs::create_dir(&node_modules).unwrap();
        
        let ts_file = temp_dir.path().join("app.ts");
        let ignored_file = node_modules.join("lib.ts");
        
        fs::write(&ts_file, "// app").unwrap();
        fs::write(&ignored_file, "// lib").unwrap();
        
        let handler = FileHandler::new(false);
        let files = handler.find_typescript_files(&[temp_dir.path().to_path_buf()]).unwrap();
        
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], ts_file);
    }

    #[test]
    fn test_create_backup() {
        let temp_dir = TempDir::new().unwrap();
        let ts_file = temp_dir.path().join("test.ts");
        let original_content = "// original content";
        fs::write(&ts_file, original_content).unwrap();
        
        let handler = FileHandler::new(true);
        handler.write_file(&ts_file, "// new content").unwrap();
        
        // Check backup was created
        let backup_file = temp_dir.path().join("test.ts.bak");
        assert!(backup_file.exists());
        assert_eq!(fs::read_to_string(&backup_file).unwrap(), original_content);
        
        // Check original file was updated
        assert_eq!(fs::read_to_string(&ts_file).unwrap(), "// new content");
    }
}