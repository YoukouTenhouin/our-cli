use crate::helpers::cli::*;
use crate::helpers::sudo;
use crate::{Error, Result};

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub struct BuildOptions {
    pub spec: PathBuf,
    pub build_root: PathBuf,
    pub rpm_dir: PathBuf,

    pub platform: String,
}

fn run_build_cmd<P: AsRef<Path>>(spec: P, build_root: P) -> Result<()> {
    let mut cmd = sudo("/usr/bin/build");
    cmd.arg("--no-checks")
        .arg("--root")
        .arg(build_root.as_ref())
        .arg(spec.as_ref());
    info!("{} {:?}", "EXEC".bold(), cmd);
    let status = cmd.status()?;
    if status.success() {
        Ok(())
    } else {
        let code = status.code().unwrap();
        Err(Error::other(format!("build command exited with {code}")))
    }
}

fn get_rpm_dir_path<S: AsRef<OsStr>>(mut build_root: PathBuf, platform: S) -> PathBuf {
    build_root.push("home/abuild/rpmbuild/RPMS");
    build_root.push(platform.as_ref());
    build_root
}

fn scan_result_rpms<P: AsRef<Path>>(rpm_dir_path: P) -> Result<Vec<PathBuf>> {
    let mut ret: Vec<PathBuf> = Vec::new();
    let paths = fs::read_dir(rpm_dir_path)?;

    for path in paths {
        let path = path?.path();
        match path.extension() {
            None => continue,
            Some(ext) => {
                if ext.to_ascii_lowercase() == "rpm" {
                    ret.push(path);
                }
            }
        }
    }

    Ok(ret)
}

fn copy_result_rpms<P: AsRef<Path>, S: AsRef<OsStr>>(
    build_root: P,
    dst_path: P,
    platform: S,
) -> Result<Vec<PathBuf>> {
    let build_rpm_dir_path = get_rpm_dir_path(build_root.as_ref().to_path_buf(), platform);
    let results = scan_result_rpms(build_rpm_dir_path)?;

    fs::create_dir_all(&dst_path)?;

    let mut ret: Vec<PathBuf> = Vec::new();
    for src in results {
        let dst = src.join(src.file_name().unwrap());
        info!(
            "{}: {} {} {}",
            "COPY".bold(),
            src.display(),
            "=>".bold(),
            dst.display()
        );
        fs::copy(src, &dst)?;
        ret.push(dst);
    }
    Ok(ret)
}

fn do_build(opts: &BuildOptions) -> Result<Vec<PathBuf>> {
    info!(
        "{} {}",
        "Building".bold(),
        opts.spec.file_name().unwrap().to_string_lossy()
    );
    run_build_cmd(&opts.spec, &opts.build_root)?;
    info!("Copying build result");
    copy_result_rpms(&opts.build_root, &opts.rpm_dir, &opts.platform)
}

pub fn build(opts: &BuildOptions) -> Result<Vec<PathBuf>> {
    do_build(opts).inspect_err(|err| err!("{} {}", "Build failed:".bold(), err))
}
