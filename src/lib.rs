mod config;
mod error;
mod updater;

pub use self::config::Config;
pub use self::error::*;

pub fn run(config: Config) -> Result<(), Error> {
    updater::update_ips(config.get_domains(), config.get_token())
}
