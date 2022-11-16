use std::time::{ Instant, Duration };

pub struct Timer {
    _start: Instant,
    _end: Instant,
    _running: bool,
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            _start: now,
            _end: now,
            _running: true,
        }
    }

    pub fn start(&mut self) {
        let now = Instant::now();
        self._start = now;
        self._end = now;
        self._running = true;
    }

    pub fn end(&mut self) {
        self._end = Instant::now();
        self._running = false;
    }

    pub fn total_s(&self) -> u64 {
        self.duration().as_secs()
    }

    pub fn total_ms(&self) -> u128 {
        self.duration().as_millis()
    }

    pub fn total_us(&self) -> u128 {
        self.duration().as_micros()
    }

    pub fn total_ns(&self) -> u128 {
        self.duration().as_nanos()
    }

    #[inline(always)]
    fn endpoint(&self) -> Instant {
        if self._running { Instant::now() } else { self._end }
    }

    #[inline(always)]
    fn duration(&self) -> Duration {
        self.endpoint().duration_since(self._start)
    }
}