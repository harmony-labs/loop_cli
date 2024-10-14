use anyhow::Result;
use clap::Parser;
use loop_lib::{parse_config, run, LoopConfig};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(trailing_var_arg = true)]
    command: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let config_path = cli.config.unwrap_or_else(|| PathBuf::from(".looprc"));
    let config = parse_config(&config_path)?;

    let command = cli.command.join(" ");
    run(&config, &command)?;

    Ok(())
}
