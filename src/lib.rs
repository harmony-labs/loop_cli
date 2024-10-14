use anyhow::{Context, Result};
use rayon::prelude::*;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
pub struct LoopConfig {
    #[serde(default)]
    pub directories: Vec<String>,
    #[serde(default)]
    pub ignore: Vec<String>,
    #[serde(default)]
    pub verbose: bool,
    #[serde(default)]
    pub silent: bool,
    #[serde(default)]
    pub parallel: bool,
}

impl Default for LoopConfig {
    fn default() -> Self {
        LoopConfig {
            directories: vec![],
            ignore: vec![".git".to_string()],
            verbose: false,
            silent: false,
            parallel: false,
        }
    }
}

pub fn run(config: &LoopConfig, command: &str) -> Result<()> {
    let dirs = expand_directories(&config.directories, &config.ignore)?;

    let run_command = |dir: &PathBuf| -> Result<()> {
        if config.verbose {
            println!("Executing in directory: {}", dir.display());
        }

        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(dir)
            .output()
            .with_context(|| format!("Failed to execute command in directory: {}", dir.display()))?;

        if !config.silent {
            println!("Status: {}", output.status);
            println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        }

        Ok(())
    };

    if config.parallel {
        dirs.par_iter().try_for_each(run_command)?;
    } else {
        dirs.iter().try_for_each(run_command)?;
    }

    Ok(())
}

fn expand_directories(directories: &[String], ignore: &[String]) -> Result<Vec<PathBuf>> {
    let mut expanded = Vec::new();

    for dir in directories {
        for entry in WalkDir::new(dir).follow_links(true).into_iter().filter_entry(|e| {
            !ignore.iter().any(|i| e.path().to_string_lossy().contains(i))
        }) {
            let entry = entry?;
            if entry.file_type().is_dir() {
                expanded.push(entry.path().to_path_buf());
            }
        }
    }

    Ok(expanded)
}

pub fn parse_config(config_path: &Path) -> Result<LoopConfig> {
    let config_str = fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read config file: {:?}", config_path))?;
    let config: LoopConfig = serde_json::from_str(&config_str)
        .with_context(|| format!("Failed to parse config file: {:?}", config_path))?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_parse_config() -> Result<()> {
        let dir = tempdir()?;
        let config_path = dir.path().join(".looprc");
        let mut file = File::create(&config_path)?;
        writeln!(file, r#"{{"ignore": [".git"], "verbose": true}}"#)?;

        let config = parse_config(&config_path)?;
        assert_eq!(config.ignore, vec![".git"]);
        assert!(config.verbose);
        Ok(())
    }

    #[test]
    fn test_default_config() -> Result<()> {
        let dir = tempdir()?;
        let config_path = dir.path().join(".looprc");
        let mut file = File::create(&config_path)?;
        writeln!(file, r#"{{}}"#)?;

        let config = parse_config(&config_path)?;
        assert_eq!(config.directories, Vec::<String>::new());
        assert_eq!(config.ignore, vec![".git"]);
        assert!(!config.verbose);
        assert!(!config.silent);
        assert!(!config.parallel);
        Ok(())
    }
}
