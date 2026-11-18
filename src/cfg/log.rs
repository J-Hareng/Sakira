use env_logger::Env;
use log::{Level, debug, error, info, log_enabled};

pub fn init() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
}
