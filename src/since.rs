use std::time::{Duration, Instant};

pub struct Timer {
    t: Instant,
}

impl Timer {
    pub fn now() -> Self {
        Timer {
            t: Instant::now(),
        }
    }

    pub fn elapsed(&mut self) -> Duration {
        let d = self.t.elapsed();
        self.t = Instant::now();
        d
    }
}
