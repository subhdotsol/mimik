// Minimal C FFI for RubberBandLiveShifter (rubberband-c.h, v4.0.0).
// The LiveShifter is designed for real-time pitch shifting with a fixed
// block size and no time-stretching, making it ideal for live voice processing.

use std::os::raw::{c_double, c_float, c_int, c_uint};

pub enum RubberBandLiveState_ {}
pub type RubberBandLiveState = *mut RubberBandLiveState_;

// Pass 0 for default options (short window, formant shifted, channels apart).
pub type RubberBandOptions = c_int;

unsafe extern "C" {
    pub fn rubberband_live_new(
        sample_rate: c_uint,
        channels: c_uint,
        options: RubberBandOptions,
    ) -> RubberBandLiveState;

    pub fn rubberband_live_delete(state: RubberBandLiveState);

    pub fn rubberband_live_reset(state: RubberBandLiveState);

    pub fn rubberband_live_set_pitch_scale(state: RubberBandLiveState, scale: c_double);

    pub fn rubberband_live_set_formant_scale(state: RubberBandLiveState, scale: c_double);

    /// Returns the fixed number of samples the shifter requires per call to shift().
    pub fn rubberband_live_get_block_size(state: RubberBandLiveState) -> c_uint;

    /// Process exactly block_size samples. input/output are arrays of per-channel pointers.
    pub fn rubberband_live_shift(
        state: RubberBandLiveState,
        input: *const *const c_float,
        output: *const *mut c_float,
    );
}
