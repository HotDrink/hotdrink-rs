use web_sys::Performance;

pub struct Timer {
    start: f64,
    prev: f64,
    times: Vec<f64>,
    perf: Performance,
}

impl Default for Timer {
    fn default() -> Self {
        let window = web_sys::window().expect("Should have a window in this context");
        let performance = window
            .performance()
            .expect("Performance should be available");
        let now = performance.now();
        Self {
            start: now,
            prev: now,
            times: Vec::new(),
            perf: performance,
        }
    }
}

impl Timer {
    /// Returns the current time in milliseconds.
    fn now(&self) -> f64 {
        self.perf.now()
    }

    /// Creates a timer that immediately starts running.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets all the timer data.
    pub fn restart(&mut self) {
        let now = self.now();
        self.start = now;
        self.prev = now;
        self.times.clear();
    }

    /// Store the current time.
    pub fn checkpoint(&mut self) {
        let now = self.now();
        let delta = now - self.prev;
        self.times.push(delta);
        self.prev = now;
    }

    /// Returns the total time passed since the timer's creation or last restart in milliseconds.
    pub fn time_since_start(&self) -> f64 {
        self.now() - self.start
    }

    /// Returns the total time passed since the last checkpoint in milliseconds.
    pub fn time_since_checkpoint(&self) -> f64 {
        self.now() - self.prev
    }

    /// Returns the average time passed between each checkpoint so far in milliseconds.
    pub fn average(&self) -> f64 {
        self.times.iter().sum::<f64>() / self.n_checkpoints() as f64
    }

    /// Returns the number of checkpoints.
    pub fn n_checkpoints(&self) -> usize {
        self.times.len()
    }

    /// Returns all deltas.
    pub fn deltas(&self) -> &[f64] {
        &self.times
    }
}
