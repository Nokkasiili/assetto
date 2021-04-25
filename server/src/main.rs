#![feature(cell_update)]
pub mod client;
pub mod config;
pub mod dynamictrack;
pub mod option;
pub mod session;
pub mod weather;
/*main.DynamicTrack.Enabled = false;
main.DynamicTrack.SessionStartGrip = 0.8;
main.DynamicTrack.BaseGrip = 0.8;
main.DynamicTrack.GripPerLap = 0.1;
main.DynamicTrack.RandomGrip = 0.0;*/

/*  ks.GetTimeMillis(puVar4,pvVar5);
local_144 = (*(int *)&main.CurrentSession->Time * 60000 -
            (int)(puVar4 + -*(int *)&main.CurrentSession->StartTime)) / 1000;*/
use anyhow::Context;
use config::Config;

const CONFIG_PATH: &str = "config.toml";
fn main() -> anyhow::Result<()> {
    println!("Loading configuration");
    let config = Config::load(CONFIG_PATH).context("failed to load configuration file")?;
    println!("{:?}", config);
    //logging::init(config.log.level);
    Ok(())
}
