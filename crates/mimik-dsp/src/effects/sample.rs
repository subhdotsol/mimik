use std::f32::consts::PI;

pub trait SampleProcessor: Send {
    fn process(&mut self, sample: f32) -> f32;
}

fn db_to_linear(db: f32) -> f32 {
    10f32.powf(db / 20.0)
}

// ── Noise gate ───────────────────────────────────────────────────────────────

pub struct NoiseGateDb {
    threshold: f32,
}

impl NoiseGateDb {
    pub fn new(threshold_db: f32) -> Self {
        Self { threshold: db_to_linear(threshold_db) }
    }
}

impl SampleProcessor for NoiseGateDb {
    fn process(&mut self, sample: f32) -> f32 {
        if sample.abs() < self.threshold { 0.0 } else { sample }
    }
}

// ── First-order high-pass filter ─────────────────────────────────────────────

pub struct HighPassFilter {
    alpha: f32,
    x_prev: f32,
    y_prev: f32,
}

impl HighPassFilter {
    pub fn new(cutoff_hz: f32, sample_rate: f32) -> Self {
        let rc = 1.0 / (2.0 * PI * cutoff_hz);
        let dt = 1.0 / sample_rate;
        let alpha = rc / (rc + dt);
        Self { alpha, x_prev: 0.0, y_prev: 0.0 }
    }
}

impl SampleProcessor for HighPassFilter {
    fn process(&mut self, x: f32) -> f32 {
        let y = self.alpha * (self.y_prev + x - self.x_prev);
        self.x_prev = x;
        self.y_prev = y;
        y
    }
}
