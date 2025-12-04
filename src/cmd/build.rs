use crate::Result;
use crate::action;
use crate::helpers::AppConfig;
use crate::helpers::cli::*;

use clap::Args;
use std::path::{Path, PathBuf};

#[derive(Args)]
pub(crate) struct CmdArgs {
    specfile: Option<String>,
}

fn find_specfile<P: AsRef<Path>>(pwd: P) -> Result<PathBuf> {
    info!("Finding specfile...");

    let mut candidates: Vec<PathBuf> = Vec::new();
    let readdir = std::fs::read_dir(&pwd)?;
    for f in readdir {
	let f = f?;
	if !f.file_type()?.is_file() {
	    continue
	}
	let path = pwd.as_ref().join(f.file_name());
	if path.extension().is_none() || path.extension().unwrap() != "spec" {
	    continue
	}

	info!("{} {}", "Candidate".bold(), path.display());
	if path.file_stem() == pwd.as_ref().file_name() {
	    return Ok(path)
	}
	candidates.push(path);
    }

    if candidates.is_empty() {
	Err(Box::new(std::io::Error::other("No spec file found")))
    } else if candidates.len() > 1 {
	Err(Box::new(std::io::Error::other(
	    "Multiple spec file found, and non of them matches package name"
	)))
    } else {
	Ok(candidates.pop().unwrap())
    }
}

pub(crate) fn run(args: &CmdArgs) -> Result<()> {
    let pwd = std::env::current_dir()?;
    let spec = if let Some(specfile) = &args.specfile {
	pwd.join(specfile)
    } else {
	find_specfile(&pwd)?
    };
    let build_root = AppConfig::global().build_root();
    let rpm_dir = AppConfig::global().rpm_dir();

    let opts = action::BuildOptions {
        spec: spec,
        build_root: build_root,
        rpm_dir: rpm_dir,

        platform: "x86_64".to_string(), // XXX hardcoded
    };

    action::prepare(pwd)?;

    let rpms = action::build(&opts)?;

    action::install(rpms)
}
