use crate::Processor;

/// Hard-clips any sample that exceeds `ceiling` in either direction.
/// Prevents digital clipping and protects downstream audio devices.
pub struct Limiter {
    pub ceiling: f32,
}

impl Limiter {
    pub fn new(ceiling: f32) -> Self {
        Self { ceiling }
    }
}

impl Processor for Limiter {
    fn process(&mut self, samples: &mut [f32]) {
        for s in samples {
            *s = s.clamp(-self.ceiling, self.ceiling);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clips_positive() {
        let mut samples = vec![1.5, 2.0];
        Limiter::new(1.0).process(&mut samples);
        assert_eq!(samples, vec![1.0, 1.0]);
    }

    #[test]
    fn clips_negative() {
        let mut samples = vec![-1.5, -2.0];
        Limiter::new(1.0).process(&mut samples);
        assert_eq!(samples, vec![-1.0, -1.0]);
    }

    #[test]
    fn passes_within_ceiling() {
        let mut samples = vec![0.5, -0.7];
        Limiter::new(1.0).process(&mut samples);
        assert_eq!(samples, vec![0.5, -0.7]);
    }
}
