use crate::Result;
use crate::action;
use crate::helpers::AppConfig;

use clap::Args;

#[derive(Args)]
pub(crate) struct CmdArgs {
    specfile: String,
}

pub(crate) fn run(args: &CmdArgs) -> Result<()> {
    let pwd = std::env::current_dir()?;
    let spec = pwd.join(&args.specfile);
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
