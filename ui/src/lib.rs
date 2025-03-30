#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(mixed_script_confusables)] // θ ehehehehe 😈

#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod coord;
pub mod spectrogram;
