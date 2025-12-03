use crate::Result;
use crate::action;

pub(crate) fn run() -> Result<()> {
    let pwd = std::env::current_dir()?;
    action::prepare(pwd)
}
