use std::sync::{Arc, RwLock};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use protocol::json::{Info, JSON};

use crate::option::ServerOptions;
use crate::{car::Cars, config::Config};

pub struct HttpServer {
    //config: Arc<Config>,
    //cars: Arc<Cars>,
}

impl HttpServer {
    pub async fn serve(
        config: Arc<Config>,
        options: Arc<RwLock<ServerOptions>>,
        cars: Arc<Cars>,
    ) -> anyhow::Result<()> {
        let addr = format!("{}:{}", config.server.address, config.server.http_port)
            .parse()
            .expect("Failed to parse http socket addrs");

        let make_service = make_service_fn(move |_| {
            let cars = cars.clone();
            let config = config.clone();
            let options = options.clone();
            async move {
                // This is the request handler.
                Ok::<_, hyper::Error>(service_fn(move |req| {
                    HttpServer::assetto(req, cars.clone(), options.clone(), config.clone())
                }))
            }
        });

        let server = Server::bind(&addr).serve(make_service);
        tokio::spawn(async move {
            if let Err(err) = server.await {
                log::error!("Server error: {}", err);
            }
        });

        log::debug!("Listening http on http://{}", addr);
        Ok(())
    }

    async fn assetto(
        req: Request<Body>,
        cars: Arc<Cars>,
        options: Arc<RwLock<ServerOptions>>,
        config: Arc<Config>,
    ) -> anyhow::Result<Response<Body>> {
        let decoded = urlencoding::decode(req.uri().path())?;
        let splitted: Vec<&str> = decoded.split("|").collect();
        log::trace!("{:?}", splitted);
        match (req.method(), splitted[0]) {
            // Serve some instructions at /
            (&Method::GET, "/") => Ok(Response::new(Body::from(""))),

            (&Method::GET, "/INFO") => {
                log::debug!("/INFO");
                Ok(Response::new(Body::from(HttpServer::info(
                    config.clone(),
                    options.clone(),
                    cars.clone(),
                ))))
            }

            (&Method::GET, "/JSON") => {
                log::debug!("/JSON");
                Ok(Response::new(Body::from(HttpServer::jsons(cars.clone()))))
            }
            _ => {
                let mut not_found = Response::default();
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        }
    }

    fn jsons(cars: Arc<Cars>) -> String {
        let p = JSON {
            cars: cars.to_json(),
        };
        serde_json::to_string(&p).unwrap()
    }

    fn info(config: Arc<Config>, options: Arc<RwLock<ServerOptions>>, cars: Arc<Cars>) -> String {
        let p = Info {
            ip: "".into(),
            port: config.server.tcp_port,
            cport: config.server.http_port,
            name: config.server.name.clone(),
            clients: cars.num_of_clients(),
            maxclients: cars.max_clients(),
            track: config.track.clone(),
            cars: cars.cars(),
            timeofday: 1337,
            session: options.read().unwrap().sessions.get_current() as u16,
            sessiontypes: options.read().unwrap().sessions.get_types(),
            durations: options.read().unwrap().sessions.get_durations(),
            timeleft: options
                .read()
                .unwrap()
                .sessions
                .left_time()
                .as_secs()
                .into(),
            country: vec!["TODO".to_string()],
            pass: config.game.password.is_some(),
            timestamp: 0,
            json: serde_json::Value::Null,
            l: false,
            pickup: true,
            tport: config.server.tcp_port,
            timed: false,
            extra: false,
            pit: false,
            inverted: 0,
        };
        serde_json::to_string(&p).unwrap()
    }
}
