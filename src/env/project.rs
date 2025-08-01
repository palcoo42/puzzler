use std::{error::Error, path::PathBuf};

/// Name of environment variable which stores path to the Cargo.toml, i.e. project root
const CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";

pub fn get_toml_path() -> Result<PathBuf, Box<dyn Error>> {
    let toml_path = std::env::var(CARGO_MANIFEST_DIR)?;
    Ok(PathBuf::from(toml_path))
}

pub fn get_project_file(path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let mut root_path = get_toml_path()?;
    root_path.push(path);
    Ok(root_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_toml_path() {
        let result = get_toml_path();

        assert!(result.is_ok());
        assert!(result.unwrap().to_string_lossy().ends_with("puzzler"));
    }

    #[test]
    fn test_get_project_file() {
        let result = get_project_file("a/b/c/file.txt");

        assert!(result.is_ok());
        assert!(
            result
                .unwrap()
                .to_string_lossy()
                .ends_with("a/b/c/file.txt")
        );
    }
}
