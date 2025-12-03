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

fn run_build<P: AsRef<Path>>(spec: P, build_root: P) -> Result<()> {
    let status = sudo("/usr/bin/build")
        .arg("--no-checks")
        .arg("--root")
        .arg(build_root.as_ref())
        .arg(spec.as_ref())
        .status()?;
    if status.success() {
        Ok(())
    } else {
        let code = status.code().unwrap();
        Err(Error::other(format!("Build process failed with {code}")))
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
    for rpm in results {
        let mut dst = dst_path.as_ref().to_path_buf();
        dst.push(rpm.file_name().unwrap());
        fs::copy(rpm, &dst)?;
        ret.push(dst);
    }
    Ok(ret)
}

pub fn build(opts: &BuildOptions) -> Result<Vec<PathBuf>> {
    run_build(&opts.spec, &opts.build_root)?;
    copy_result_rpms(&opts.build_root, &opts.rpm_dir, &opts.platform)
}
