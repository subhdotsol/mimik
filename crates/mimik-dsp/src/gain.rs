use crate::Processor;

/// Multiplies every sample by `factor`. Values above 1.0 amplify; below 1.0 attenuate.
pub struct Gain {
    pub factor: f32,
}

impl Gain {
    pub fn new(factor: f32) -> Self {
        Self { factor }
    }
}

impl Processor for Gain {
    fn process(&mut self, samples: &mut [f32]) {
        for s in samples {
            *s *= self.factor;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amplifies() {
        let mut samples = vec![0.5, -0.5, 1.0];
        Gain::new(2.0).process(&mut samples);
        assert_eq!(samples, vec![1.0, -1.0, 2.0]);
    }

    #[test]
    fn attenuates() {
        let mut samples = vec![1.0, -1.0];
        Gain::new(0.5).process(&mut samples);
        assert_eq!(samples, vec![0.5, -0.5]);
    }
}
