use anyhow::Result;
use loop_lib::{should_ignore, LoopConfig};
use rayon::prelude::*;
use std::path::PathBuf;
use std::process::Command;


pub fn run(config: &LoopConfig, command: &str) -> Result<()> {
    let dirs = expand_directories(&config.directories, &config.ignore)?;

    if config.parallel {
        dirs.par_iter().for_each(|dir| {
            run_command(&PathBuf::from(dir), command, config.verbose).unwrap();
        });
    } else {
        for dir in dirs {
            run_command(&PathBuf::from(&dir), command, config.verbose)?;
        }
    }

    Ok(())
}

fn run_command(dir: &PathBuf, command: &str, verbose: bool) -> Result<()> {
    if verbose {
        println!("Executing in directory: {}", dir.display());
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .current_dir(dir)
        .output()?;

    if !output.status.success() {
        anyhow::bail!("Command failed in directory: {}", dir.display());
    }

    Ok(())
}

pub fn expand_directories(directories: &[String], ignore: &[String]) -> Result<Vec<String>> {
    let mut expanded = Vec::new();

    use std::fs;

    for dir in directories {
        let dir_path = PathBuf::from(dir);
        if dir_path.is_dir() && !should_ignore(&dir_path, ignore) {
            expanded.push(dir_path.to_string_lossy().into_owned());

            for entry in fs::read_dir(&dir_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() && !should_ignore(&path, ignore) {
                    expanded.push(path.to_string_lossy().into_owned());
                }
            }
        }
    }

    Ok(expanded)
}

pub fn parse_config(config_path: &PathBuf) -> Result<LoopConfig> {
    let config_str = std::fs::read_to_string(config_path)?;
    let config: LoopConfig = serde_json::from_str(&config_str)?;
    Ok(config)
}
