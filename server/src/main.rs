pub mod config;
pub mod option;
use config::Config;
use anyhow::Context;

const CONFIG_PATH: &str = "config.toml";
fn main() -> anyhow::Result<()> {
    println!("Loading configuration");
    let config = Config::load(CONFIG_PATH).context("failed to load configuration file")?;
    println!("{:?}", config);
    //logging::init(config.log.level);
    Ok(())
}
