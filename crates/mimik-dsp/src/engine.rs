use crate::{Chain, Processor};

/// The top-level DSP engine. Owns a `Chain` and exposes a single `process` call.
pub struct Engine {
    chain: Chain,
}

impl Engine {
    pub fn new(chain: Chain) -> Self {
        Self { chain }
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        self.chain.process(samples);
    }
}
