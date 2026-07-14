use crate::ffi;

/// Common interface for streaming audio effects.
pub trait AudioEffect: Send {
    fn process(&mut self, input: &[f32], output: &mut Vec<f32>);
    fn reset(&mut self);
}

/// Real-time pitch shifter backed by RubberBandLiveShifter.
///
/// Always processes exactly `block_size()` samples per call to `shift()`.
/// The pipeline is responsible for accumulating samples into full blocks
/// before calling shift().
pub struct PitchShifter {
    state: ffi::RubberBandLiveState,
    block_size: usize,
    semitones: f32,
    pitch_ratio: f64,
}

impl PitchShifter {
    pub fn new(sample_rate: u32, semitones: f32) -> Self {
        let pitch_ratio = 2_f64.powf(semitones as f64 / 12.0);

        let state = unsafe { ffi::rubberband_live_new(sample_rate, 1, 0) };
        unsafe { ffi::rubberband_live_set_pitch_scale(state, pitch_ratio) };
        let block_size = unsafe { ffi::rubberband_live_get_block_size(state) as usize };

        Self {
            state,
            block_size,
            semitones,
            pitch_ratio,
        }
    }

    pub fn block_size(&self) -> usize {
        self.block_size
    }

    pub fn semitones(&self) -> f32 {
        self.semitones
    }

    pub fn pitch_ratio(&self) -> f64 {
        self.pitch_ratio
    }

    /// Shift exactly `block_size()` samples. Panics in debug if lengths mismatch.
    pub fn shift(&mut self, input: &[f32], output: &mut [f32]) {
        debug_assert_eq!(input.len(), self.block_size);
        debug_assert_eq!(output.len(), self.block_size);

        let input_ptr: *const f32 = input.as_ptr();
        let output_ptr: *mut f32 = output.as_mut_ptr();

        unsafe {
            ffi::rubberband_live_shift(self.state, &input_ptr, &output_ptr);
        }
    }
}

impl Drop for PitchShifter {
    fn drop(&mut self) {
        unsafe { ffi::rubberband_live_delete(self.state) };
    }
}

// RubberBandLiveShifter is not thread-safe for concurrent access, but we only
// ever move it into a single processing thread, so Send is safe.
unsafe impl Send for PitchShifter {}
