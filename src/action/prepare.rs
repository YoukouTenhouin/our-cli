use crate::Result;
use crate::helpers::AppConfig;
use crate::helpers::cli::*;

use serde::Deserialize;
use serde_xml_rs::from_reader;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir_in;

#[derive(Deserialize)]
#[serde(rename = "services")]
struct ServiceFile {
    #[serde(rename = "service")]
    services: Vec<Service>,
}

#[derive(Deserialize)]
struct Service {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@mode")]
    mode: Option<String>,
    #[serde(rename = "param", default = "Vec::default")]
    params: Vec<Param>,
}

#[derive(Deserialize)]
struct Param {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "#text")]
    value: String,
}

fn parse_service_defs(file: &fs::File) -> Result<ServiceFile> {
    Ok(from_reader(file)?)
}

fn run_service<P: AsRef<Path>>(srv: &Service, pwd: P) -> Result<()> {
    let tempdir = tempdir_in(&pwd)?;

    let cmd_path = AppConfig::global().obs_service_root().join(&srv.name);
    let mut cmd = Command::new(cmd_path);
    cmd.arg("--outdir").arg(tempdir.path().as_os_str());
    for p in &srv.params {
        cmd.arg(format!("--{}", p.name));
        cmd.arg(&p.value);
    }
    info!("{} {:?}", "EXEC".bold(), cmd);
    let status = cmd.status()?;
    if !status.success() {
        let code = status.code().unwrap();
        return Err(Box::new(std::io::Error::other(format!(
            "service {} exited with {code}",
            srv.name
        ))));
    }

    let read_dir = fs::read_dir(tempdir.path())?;
    for f in read_dir {
        let f = f?;
        info!("{} {}", "MOVE".bold(), f.file_name().display());
        fs::rename(f.path(), pwd.as_ref().join(f.file_name()))?;
    }
    Ok(())
}

fn try_run_all_services<P: AsRef<Path>>(pwd: P) -> Result<()> {
    let service_defs_path = pwd.as_ref().join("_service");
    if !fs::exists(&service_defs_path)? {
        info!("_service not found in current directory. Skip.");
        return Ok(());
    };

    let file = fs::File::open(service_defs_path)?;
    let service_file = parse_service_defs(&file)?;
    info!("{}", "Running services".bold());

    for s in service_file.services {
        match s.mode.as_ref().map(|m| m.as_str()) {
            None | Some("trylocal") | Some("localonly") => {
                info!("{} {}", "Running".bold(), s.name);
                run_service(&s, &pwd)?;
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn prepare<P: AsRef<Path>>(pwd: P) -> Result<()> {
    try_run_all_services(pwd)
}
