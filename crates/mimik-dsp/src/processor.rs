/// A single DSP effect that transforms audio samples in place.
pub trait Processor: Send {
    fn process(&mut self, samples: &mut [f32]);
}
