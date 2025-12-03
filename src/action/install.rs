use crate::Result;
use crate::helpers::sudo;

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
        Err(Box::new(std::io::Error::other(format!(
            "zypper exited with status {code}"
        ))))
    }
}
