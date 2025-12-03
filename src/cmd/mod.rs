mod build;
mod prepare;

use crate::Result;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build(build::CmdArgs),
    Prepare,
}

pub(crate) fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build(args) => build::run(&args),
        Commands::Prepare => prepare::run(),
    }
}
