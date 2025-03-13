use std::path::PathBuf;

const APP_DIR: &str = ".hn";
const LOG_DIR: &str = "log";
const CACHE_DIR: &str = "cache";
const DB_DIR: &str = "db";
const CONFIG_DIR: &str = "config";

#[allow(unused)]
#[inline]
pub(crate) fn app_dir() -> PathBuf {
    dirs::home_dir()
        .expect("failed to get home directory")
        .join(APP_DIR)
}

#[allow(unused)]
#[inline]
pub(crate) fn log_dir() -> PathBuf {
    dirs::home_dir()
        .expect("failed to get home directory")
        .join(LOG_DIR)
}

#[allow(unused)]
#[inline]
pub(crate) fn db_dir() -> PathBuf {
    dirs::home_dir()
        .expect("failed to get home directory")
        .join(DB_DIR)
}

#[allow(unused)]
#[inline]
pub(crate) fn cache_dir() -> PathBuf {
    dirs::home_dir()
        .expect("failed to get home directory")
        .join(CACHE_DIR)
}

#[allow(unused)]
#[inline]
pub(crate) fn config_dir() -> PathBuf {
    dirs::home_dir()
        .expect("failed to get home directory")
        .join(CONFIG_DIR)
}
