#![deny(clippy::all)]

#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod dsp;
pub mod pitch;
pub mod stream;
pub mod synth;

use approx::{AbsDiffEq, RelativeEq};
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

impl AbsDiffEq for Hz {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        other.0.abs_diff_eq(&other.0, epsilon)
    }
}

impl RelativeEq for Hz {
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }

    fn relative_eq(
        &self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon
    ) -> bool {
        self.0.relative_eq(&other.0, epsilon, max_relative)
    }
}
