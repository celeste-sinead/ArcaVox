use std::ops::{Add, Sub};
use std::time;

use cpal::{self};

pub mod buffer;
pub mod executor;
pub mod input;
pub mod output;
pub mod pipeline;
pub mod transform;
pub mod wav;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct ChannelCount(u16);

impl ChannelCount {
    pub fn new(c: u16) -> ChannelCount {
        ChannelCount(c)
    }
}

impl From<ChannelCount> for u16 {
    fn from(v: ChannelCount) -> u16 {
        v.0
    }
}

impl From<ChannelCount> for usize {
    fn from(v: ChannelCount) -> usize {
        v.0 as usize
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SampleRate(u32);

impl SampleRate {
    pub fn new(s: u32) -> SampleRate {
        SampleRate(s)
    }
}

impl From<SampleRate> for u32 {
    fn from(v: SampleRate) -> u32 {
        v.0
    }
}

impl From<SampleRate> for usize {
    fn from(v: SampleRate) -> usize {
        v.0 as usize
    }
}

impl From<SampleRate> for f32 {
    fn from(v: SampleRate) -> f32 {
        v.0 as f32
    }
}

impl From<SampleRate> for cpal::SampleRate {
    fn from(v: SampleRate) -> cpal::SampleRate {
        cpal::SampleRate(v.0)
    }
}

/// Represents a point in time, in seconds, in a signal
/// Essentially the same as std::time::Instant, but the latter is unusably
/// opaque.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Instant{
    sample_index: usize,
    sample_rate: SampleRate,
}

impl Instant {
    pub fn new(sample_index: usize, sample_rate: SampleRate) -> Instant {
        Instant{sample_index, sample_rate}
    }

    pub fn as_secs_from_start_f32(self: Instant) -> f32 {
        Duration::from_start(self).as_secs_f32()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Duration{
    sample_count: usize,
    sample_rate: SampleRate,
}

impl Duration {
    pub fn new(sample_count: usize, sample_rate: SampleRate) -> Duration {
        Duration{sample_count, sample_rate}
    }

    pub fn from_start(i: Instant) -> Duration {
        Duration{sample_count: i.sample_index, sample_rate: i.sample_rate}
    }

    pub fn as_secs_f32(self: Duration) -> f32 {
        time::Duration::from(self).as_secs_f32()
    }
}

impl From<Duration> for time::Duration {
    fn from(stream_dur: Duration) -> Self {
        let secs= stream_dur.sample_count as u64 / stream_dur.sample_rate.0 as u64;
        let remain= stream_dur.sample_count % usize::from(stream_dur.sample_rate);
        let nanos = remain * 1000 * 1000 * 1000 / usize::from(stream_dur.sample_rate);
        time::Duration::new(secs, nanos as u32)
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        Duration::new(rhs.sample_index.checked_sub(self.sample_index).unwrap(), self.sample_rate)
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, rhs: Duration) -> Instant {
        Instant::new(self.sample_index.checked_sub(rhs.sample_count).unwrap(), self.sample_rate)
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, rhs: Duration) -> Instant {
        Instant::new(self.sample_index + rhs.sample_count, self.sample_rate)
    }
}


/// A batch of samples received from an input device.
pub struct Frame {
    pub channels: ChannelCount,
    pub sample_rate: SampleRate,
    pub samples: Vec<f32>,
}
