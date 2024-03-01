use std::cell::Cell;
use std::ops::Sub;
use std::time::{Duration, Instant};

use crate::config::Session as CfgSession;
use protocol::packets::server::SessionType;

use protocol::packets::server::SessionU as SessionPacket;
#[derive(Clone, Debug)]
pub enum SessionOpenType {
    Closed,
    Open,
    WaitOnly,
}

#[derive(Clone, Debug)]
pub struct Session {
    pub name: String,
    pub session_type: SessionType,
    pub end: Duration,
    pub laps: u16,
    //  wait: Duration,
    //    open_type: SessionOpenType,
}

#[derive(Clone, Debug)]
pub struct Sessions {
    sessions: Vec<Session>,
    start: Instant,
    current: usize,
}

impl Sessions {
    pub fn get_types(&self) -> Vec<u8> {
        self.sessions
            .iter()
            .map(|f| f.session_type.clone() as u8)
            .collect()
    }
    pub fn get_durations(&self) -> Vec<i64> {
        self.sessions
            .iter()
            .map(|f| f.end.as_secs() as i64)
            .collect()
    }

    pub fn get_current_session(&self) -> &Session {
        self.sessions.get(self.get_current()).unwrap()
    }
    pub fn get_current(&self) -> usize {
        self.current
    }

    pub fn is_over(&self) -> bool {
        self.start.elapsed() >= self.get_current_session().end
    }
    pub fn left_time(&self) -> Duration {
        self.get_current_session()
            .end
            .checked_sub(self.start.elapsed())
            .unwrap_or(Duration::default())
    }

    pub fn start(&mut self) {
        self.start = Instant::now();
    }
    pub fn get_start(&self) -> Instant {
        self.start
    }

    pub fn next_session(&mut self) {
        self.current = self.get_current() + 1 % self.sessions.len();
        self.start();
    }
}

impl From<CfgSession> for Session {
    fn from(s: CfgSession) -> Self {
        Self {
            name: s.name,
            session_type: s.session_type.into(),
            end: Duration::new(s.time.into(), 0),
            laps: s.laps,
            // wait: s.wait,
            // open_type: s.open_type,
        }
    }
}

impl From<&Vec<CfgSession>> for Sessions {
    fn from(s: &Vec<CfgSession>) -> Self {
        let mut sessions: Vec<Session> = Vec::new();

        for i in s.into_iter() {
            sessions.push(i.clone().into())
        }

        Self {
            sessions,
            start: Instant::now(),
            current: 0,
        }
    }
}

impl From<Sessions> for Vec<SessionPacket> {
    fn from(sessions: Sessions) -> Self {
        let mut ret: Self = Vec::new();
        for i in sessions.sessions.iter() {
            ret.push(SessionPacket {
                session_type: i.session_type.clone(),
                laps: i.laps,
                time: i.end.as_secs() as u16,
            })
        }
        ret
    }
}
