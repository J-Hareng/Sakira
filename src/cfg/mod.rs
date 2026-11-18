use ::log::info;

mod env;
mod log;

pub fn init() {
    env::init();
    log::init();

    info!("Initialized environment and logging :) ")
}
