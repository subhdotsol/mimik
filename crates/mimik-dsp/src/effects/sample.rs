use std::f32::consts::PI;
use std::f32::consts::TAU;

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

// ── Compressor with attack/release ───────────────────────────────────────────

pub struct SmoothCompressor {
    threshold: f32,
    slope: f32,
    attack_coeff: f32,
    release_coeff: f32,
    envelope: f32,
}

impl SmoothCompressor {
    pub fn new(
        threshold_db: f32,
        ratio: f32,
        attack_ms: f32,
        release_ms: f32,
        sample_rate: f32,
    ) -> Self {
        Self {
            threshold: db_to_linear(threshold_db),
            slope: 1.0 - 1.0 / ratio,
            attack_coeff: 1.0 - (-2.2 / (sample_rate * attack_ms / 1000.0)).exp(),
            release_coeff: 1.0 - (-2.2 / (sample_rate * release_ms / 1000.0)).exp(),
            envelope: 0.0,
        }
    }
}

impl SampleProcessor for SmoothCompressor {
    fn process(&mut self, sample: f32) -> f32 {
        let level = sample.abs();
        let coeff = if level > self.envelope { self.attack_coeff } else { self.release_coeff };
        self.envelope += (level - self.envelope) * coeff;

        let gain = if self.envelope > self.threshold {
            let over_db = 20.0 * (self.envelope / self.threshold).log10();
            db_to_linear(-over_db * self.slope)
        } else {
            1.0
        };

        sample * gain
    }
}

// ── Hard limiter ─────────────────────────────────────────────────────────────

pub struct LimiterDb {
    ceiling: f32,
}

impl LimiterDb {
    pub fn new(ceiling_db: f32) -> Self {
        Self { ceiling: db_to_linear(ceiling_db) }
    }
}

impl SampleProcessor for LimiterDb {
    fn process(&mut self, sample: f32) -> f32 {
        sample.clamp(-self.ceiling, self.ceiling)
    }
}

// ── Peaking EQ (biquad, Audio EQ Cookbook) ───────────────────────────────────

pub struct PeakingEq {
    b0: f32, b1: f32, b2: f32,
    a1: f32, a2: f32,
    x1: f32, x2: f32,
    y1: f32, y2: f32,
}

impl PeakingEq {
    pub fn new(freq_hz: f32, gain_db: f32, q: f32, sample_rate: f32) -> Self {
        let a = 10f32.powf(gain_db / 40.0);
        let omega = 2.0 * PI * freq_hz / sample_rate;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / (2.0 * q);

        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cs;
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        let a1 = -2.0 * cs;
        let a2 = 1.0 - alpha / a;

        Self {
            b0: b0 / a0, b1: b1 / a0, b2: b2 / a0,
            a1: a1 / a0, a2: a2 / a0,
            x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0,
        }
    }
}

impl SampleProcessor for PeakingEq {
    fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.b1 * self.x1 + self.b2 * self.x2
              - self.a1 * self.y1 - self.a2 * self.y2;
        self.x2 = self.x1; self.x1 = x;
        self.y2 = self.y1; self.y1 = y;
        y
    }
}
