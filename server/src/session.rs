use std::cell::Cell;
use std::time::{Duration, Instant};

use protocol::packets::server::SessionType;

pub enum SessionOpenType {
    Closed,
    Open,
    WaitOnly,
}

pub struct Session {
    name: String,
    session_type: SessionType,
    start: Cell<Instant>,
    end: Duration,
    wait: Duration,
    open_type: SessionOpenType,
}

impl Session {
    /*pub fn new(name: &str) -> Self {
    Self { name: name.into()
    }
    }*/
    pub fn is_over(&self) -> bool {
        self.start.get().elapsed() >= self.end
    }
    pub fn start(&self) {
        self.start.set(Instant::now());
    }
}
