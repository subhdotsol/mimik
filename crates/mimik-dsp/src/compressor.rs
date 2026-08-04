use crate::Processor;

/// Reduces the dynamic range of the signal.
///
/// Samples louder than `threshold` have the excess reduced by `ratio`.
/// A ratio of 4.0 means 4 dB of input above threshold produces 1 dB of output above threshold.
pub struct Compressor {
    pub threshold: f32,
    pub ratio: f32,
}

impl Compressor {
    pub fn new(threshold: f32, ratio: f32) -> Self {
        Self { threshold, ratio }
    }
}

impl Processor for Compressor {
    fn process(&mut self, samples: &mut [f32]) {
        for s in samples {
            let abs = s.abs();
            if abs > self.threshold {
                let excess = abs - self.threshold;
                let compressed = self.threshold + excess / self.ratio;
                *s = compressed * s.signum();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compresses_loud_samples() {
        let mut samples = vec![0.9];
        // threshold=0.5, ratio=4 → excess=0.4 → compressed=0.5 + 0.4/4 = 0.6
        Compressor::new(0.5, 4.0).process(&mut samples);
        assert!((samples[0] - 0.6).abs() < 1e-6);
    }

    #[test]
    fn leaves_quiet_samples() {
        let mut samples = vec![0.3, -0.2];
        Compressor::new(0.5, 4.0).process(&mut samples);
        assert_eq!(samples, vec![0.3, -0.2]);
    }

    #[test]
    fn preserves_sign() {
        let mut samples = vec![-0.9];
        Compressor::new(0.5, 4.0).process(&mut samples);
        assert!(samples[0] < 0.0);
    }
}
