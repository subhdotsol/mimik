use crate::Processor;

/// Runs a sequence of processors on the same buffer in order.
pub struct Chain {
    processors: Vec<Box<dyn Processor>>,
}

impl Chain {
    pub fn new() -> Self {
        Self {
            processors: Vec::new(),
        }
    }

    pub fn add(&mut self, processor: impl Processor + 'static) -> &mut Self {
        self.processors.push(Box::new(processor));
        self
    }
}

impl Default for Chain {
    fn default() -> Self {
        Self::new()
    }
}

impl Processor for Chain {
    fn process(&mut self, samples: &mut [f32]) {
        for p in &mut self.processors {
            p.process(samples);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Gain, Limiter};

    #[test]
    fn applies_in_order() {
        let mut chain = Chain::new();
        chain.add(Gain::new(3.0));  // 0.5 → 1.5
        chain.add(Limiter::new(1.0)); // 1.5 → 1.0

        let mut samples = vec![0.5];
        chain.process(&mut samples);
        assert_eq!(samples, vec![1.0]);
    }
}
