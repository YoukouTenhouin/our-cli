use std::ffi::OsStr;
use std::process::Command;

pub(crate) fn sudo<S: AsRef<OsStr>>(program: S) -> Command {
    let mut command = Command::new("sudo");
    command.arg(program);
    command
}
