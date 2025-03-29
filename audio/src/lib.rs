#![deny(clippy::all)]

#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod dsp;
pub mod pitch;
pub mod stream;
pub mod synth;

use stream::input::Instant;
pub use stream::transform::FFTResult;

#[derive(Clone, Debug)]
pub struct RMSLevels {
    /// The end time of the measurement period
    pub time: Instant,
    /// Full scale RMS, for each channel
    pub values: Vec<f32>,
}

// The message type that is used to update iced application state
#[derive(Debug, Clone)]
pub enum Message {
    AudioStreamClosed,
    FFTResult(FFTResult),
    RMSLevels(RMSLevels),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hz(pub f32);

impl From<Hz> for f32 {
    fn from(v: Hz) -> f32 {
        v.0
    }
}

