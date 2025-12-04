mod build;
mod prepare;

use crate::helpers::cli::*;

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

pub(crate) fn run() {
    let cli = Cli::parse();

    let (cmd, ret) = match &cli.command {
        Commands::Build(args) => ("build", build::run(&args)),
        Commands::Prepare => ("prepare", prepare::run()),
    };

    ret.unwrap_or_else(|e| {
	err!("{} {} {} {}", "Command".bold(), cmd, "failed with: ".bold(), e);
	std::process::exit(1)
    });
}
