use crate::helpers::sudo;
use crate::{Error, Result};

use std::path::Path;

pub fn install<I, P>(rpms: I) -> Result<()>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let status = sudo("zypper")
        .arg("--no-gpg-checks")
        .arg("in")
        .args(rpms.into_iter().map(|p| p.as_ref().to_path_buf()))
        .status()?;
    if status.success() {
        Ok(())
    } else {
        let code = status.code().unwrap();
        Err(Error::other(format!("zypper exited with status {code}")))
    }
}
