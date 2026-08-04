use crate::Processor;

/// Silences any sample whose absolute value falls below `threshold`.
/// Removes background hiss and room noise during silence.
pub struct NoiseGate {
    pub threshold: f32,
}

impl NoiseGate {
    pub fn new(threshold: f32) -> Self {
        Self { threshold }
    }
}

impl Processor for NoiseGate {
    fn process(&mut self, samples: &mut [f32]) {
        for s in samples {
            if s.abs() < self.threshold {
                *s = 0.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silences_below_threshold() {
        let mut samples = vec![0.005, -0.003, 0.5, -0.8];
        NoiseGate::new(0.01).process(&mut samples);
        assert_eq!(samples, vec![0.0, 0.0, 0.5, -0.8]);
    }

    #[test]
    fn passes_above_threshold() {
        let mut samples = vec![0.1, -0.2];
        NoiseGate::new(0.01).process(&mut samples);
        assert_eq!(samples, vec![0.1, -0.2]);
    }
}
