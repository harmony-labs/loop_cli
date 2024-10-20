use anyhow::Result;
use loop_lib::{expand_directories, LoopConfig};
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

pub fn parse_config(config_path: &PathBuf) -> Result<LoopConfig> {
    let config_str = std::fs::read_to_string(config_path)?;
    let config: LoopConfig = serde_json::from_str(&config_str)?;
    Ok(config)
}
