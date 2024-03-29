#![feature(cell_update)]
pub mod car;
pub mod client;
pub mod config;
pub mod dynamictrack;
pub mod http;
pub mod listener;
pub mod option;
pub mod readwrite;
pub mod server;
pub mod session;
pub mod system;
pub mod tickloop;
pub mod udpserver;
pub mod weather;
/*main.DynamicTrack.Enabled = false;
main.DynamicTrack.SessionStartGrip = 0.8;
main.DynamicTrack.BaseGrip = 0.8;
main.DynamicTrack.GripPerLap = 0.1;
main.DynamicTrack.RandomGrip = 0.0;*/

use crate::http::HttpServer;
use crate::udpserver::UdpServer;
use std::borrow::BorrowMut;

/*  ks.GetTimeMillis(puVar4,pvVar5);
local_144 = (*(int *)&main.CurrentSession->Time * 60000 -
            (int)(puVar4 + -*(int *)&main.CurrentSession->StartTime)) / 1000;*/
use crate::option::ServerOptions;
use crate::{server::Server, tickloop::TickLoop};
use anyhow::Context;
use config::Config;

const CONFIG_PATH: &str = "config.toml";
use crate::car::Cars;

use std::sync::Arc;
pub struct Game {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::debug!("Loading configuration");
    let config = Config::load(CONFIG_PATH).context("failed to load configuration file")?;
    log::debug!("{:?}", config);
    env_logger::builder().filter_level(config.log.level).init();

    let config = Arc::new(config);
    let cars = Arc::new(Cars::new(Arc::clone(&config)));
    let mut options = ServerOptions::new(Arc::clone(&config));
    options.borrow_mut().write().unwrap().update_weather();
    let udp_server = UdpServer::bind(Arc::clone(&config)).await?;

    let mut server = Server::bind(
        config.clone(),
        cars.clone(),
        options.clone(),
        udp_server.received_packets(),
        udp_server.packets_to_send(),
    )
    .await?;

    let http_server = HttpServer::serve(config.clone(), options.clone(), cars.clone()).await?;

    let tickloop = TickLoop::new(config.server.client_send_interval_hz.into(), move || {
        udp_server.listen();
        udp_server.send_udp();
        server.accept_new_players();
        server.handle_udp_messages();
        server.handle_tcp_packets();
        server.send_pings_and_updates();

        //log::debug!("tick");
        false
    });
    tickloop.run();
    Ok(())
}
