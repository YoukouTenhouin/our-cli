use super::xdg;

use config::Config;
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::OnceLock;

static CONFIG_CELL: OnceLock<AppConfig> = OnceLock::new();

fn config_file_base_name() -> Option<PathBuf> {
    xdg::config_home().map(|mut p| {
        p.push("our/config");
        p
    })
}

#[derive(Deserialize)]
pub(crate) struct AppConfig {
    build_root: Option<PathBuf>,
    rpm_dir: Option<PathBuf>,
    osc_service_root: Option<PathBuf>,
}

impl AppConfig {
    fn load() -> Self {
        let builder = if let Some(base_name) = config_file_base_name() {
            Config::builder().add_source(
                config::File::with_name(base_name.to_string_lossy().as_ref()).required(false),
            )
        } else {
            Config::builder()
        };
        let config = builder
            .add_source(config::Environment::with_prefix("OUR"))
            .build()
            .unwrap();
        config
            .try_deserialize()
            .expect("error loading configuration")
    }

    pub(crate) fn global() -> &'static Self {
        CONFIG_CELL.get_or_init(AppConfig::load)
    }

    pub(crate) fn build_root(&self) -> PathBuf {
        self.build_root
            .clone()
            .unwrap_or(PathBuf::from("/var/tmp/our/build-root"))
    }

    pub(crate) fn rpm_dir(&self) -> PathBuf {
        self.rpm_dir
            .clone()
            .or_else(|| {
                xdg::cache_home().map(|mut p| {
                    p.push("our/rpms");
                    p
                })
            })
            .expect("rpm directory not set, and unable to determine automatically")
    }

    pub(crate) fn obs_service_root(&self) -> PathBuf {
        self.osc_service_root
            .clone()
            .unwrap_or_else(|| PathBuf::from("/usr/lib/obs/service"))
    }
}
