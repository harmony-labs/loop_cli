use anyhow::Result;
use clap::Parser;
use loop_lib::{expand_directories, parse_config, run, LoopConfig};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, help = "Specify directories to include (overrides config file)")]
    include: Option<Vec<String>>,

    #[arg(short, long, help = "Specify directories to exclude (adds to config file exclusions)")]
    exclude: Option<Vec<String>>,

    #[arg(short, long, help = "Enable verbose output")]
    verbose: bool,

    #[arg(short, long, help = "Enable silent mode (suppress all output)")]
    silent: bool,

    #[arg(long, help = "Execute commands in parallel")]
    parallel: bool,

    #[arg(long, help = "Add shell aliases to the global .looprc file")]
    add_aliases_to_global_looprc: bool,

    #[arg(trailing_var_arg = true)]
    command: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let config_path = cli.config.unwrap_or_else(|| PathBuf::from(".looprc"));
    let mut config = if config_path.exists() {
        parse_config(&config_path)?
    } else {
        LoopConfig::default()
    };

    // Update config with CLI options
    if let Some(include) = cli.include {
        config.directories = include;
    }
    if let Some(exclude) = cli.exclude {
        config.ignore.extend(exclude);
    }

    config.verbose = cli.verbose;
    config.silent = cli.silent;
    config.parallel = cli.parallel;
    config.add_aliases_to_global_looprc = cli.add_aliases_to_global_looprc;

    // If no directories specified, use current and all child directories
    if config.directories.is_empty() {
        let dirs = expand_directories(&[".".to_string()], &config.ignore)?;
        config.directories = dirs;
    }

    if cli.command.is_empty() {
        Cli::command().print_help()?;
        return Ok(());
    }
    let command = cli.command.join(" ");
    run(&config, &command)?;

    Ok(())
}
