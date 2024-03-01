use std::time::Duration;
use std::time::Instant;

pub struct TickLoop {
    function: Box<dyn FnMut() -> bool>,
    tick_duration: Duration,
}

impl TickLoop {
    /// Creates a `TickLoop`. The given `function` is called
    /// each tick. Returning `true` from `function` causes the
    /// tick loop to exit.
    pub fn new(tick_rate: u32, function: impl FnMut() -> bool + 'static) -> Self {
        let tick_millis: u32 = 1000 / tick_rate;
        let tick_duration: Duration = Duration::from_millis(tick_millis as u64);
        Self {
            function: Box::new(function),
            tick_duration,
        }
    }

    /// Runs the tick loop until the callback returns `true`.
    pub fn run(mut self) {
        loop {
            let start = Instant::now();
            let should_exit = (self.function)();
            if should_exit {
                return;
            }

            let elapsed = start.elapsed();
            if elapsed > self.tick_duration {
                log::warn!("Tick took too long ({:?})", elapsed);
            } else {
                std::thread::sleep(self.tick_duration - elapsed);
            }
        }
    }
}
