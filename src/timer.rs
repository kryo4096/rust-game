use std::time::{Instant, Duration};

pub struct Timer {
    start: Instant,
    last_frame: Instant,
    delta: f32,
}

impl Timer {
    pub fn start() -> Self {
        let start = Instant::now();
        let delta = Duration::from_secs(0);

        Self {start, last_frame: start, delta: 0.}
    }

    pub fn update(&mut self) {
        self.delta = duration_to_secs(self.last_frame.elapsed());

        self.last_frame = Instant::now();
    }

    pub fn delta(&self) -> f32 {
        self.delta
    }

    pub fn time(&self) -> f32 {
        duration_to_secs(self.start.elapsed())
    }
}

fn duration_to_secs(duration: Duration) -> f32 {
    duration.as_secs() as f32 + duration.subsec_nanos() as f32 / 1e9
}
