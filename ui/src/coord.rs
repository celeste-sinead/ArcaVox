//! Definitions of co-ordinate spaces:
//! - Subject space: the co-ordinate space relative to what's displayed, e.g.
//!   units of time, frequency, amplitude, etc.
//! - View space: normalized within a viewport; visible coordinates in
//!   [-1.0, 1.0]. Right-handed, i.e. + is right/up
//! - Screen space: pixel coordinates. Left-handed, i.e. + is right/down

use std::ops::Sub;

use iced::Rectangle;

use audio::stream::{Duration, Instant, Period};

pub trait Transform<From, To> {
    fn transform(&self, val: From) -> To;

    /// i.e. inverse(transform(x)) == x
    fn inverse(&self, val: To) -> From;
}

pub struct Linear {
    slope: f32,
    offset: f32,
}

impl Linear {
    /// Create a pair of 1D transforms to map view space to the given
    /// rectangular viewport
    #[must_use]
    pub fn make_screen(viewport: Rectangle) -> (Linear, Linear) {
        let x = Linear {
            slope: viewport.width / 2.0,
            offset: viewport.width / 2.0 + viewport.x,
        };
        let y = Linear {
            slope: -(viewport.height / 2.0),
            offset: (viewport.height / 2.0) + viewport.y,
        };
        (x, y)
    }
}

impl Transform<f32, f32> for Linear {
    fn transform(&self, val: f32) -> f32 {
        self.slope * val + self.offset
    }

    fn inverse(&self, val: f32) -> f32 {
        (val - self.offset) / self.slope
    }
}

pub struct InstantView {
    period: Period,
}

impl InstantView {
    #[must_use]
    pub fn new(period: Period) -> Self {
        Self { period }
    }
}

#[allow(
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
impl Transform<Instant, f32> for InstantView {
    fn transform(&self, val: Instant) -> f32 {
        let delta = if val < self.period.start() {
            // Duration cannot be negative, so need to reverse and negate
            -(self.period.start().sub(val).sample_count() as isize)
        } else {
            val.sub(self.period.start()).sample_count() as isize
        };
        let frac = delta as f32 / self.period.duration().sample_count() as f32;
        2.0 * frac - 1.0
    }

    fn inverse(&self, val: f32) -> Instant {
        let frac = (val + 1.0) / 2.0;
        let delta = (frac * self.period.duration().sample_count() as f32).round() as isize;
        if delta >= 0 {
            self.period.start() + Duration::new(delta as usize, self.period.sample_rate())
        } else {
            self.period.start() - Duration::new(-delta as usize, self.period.sample_rate())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use audio::stream::SampleRate;
    use iced::{Point, Size};

    #[test]
    fn screen_transform() {
        let (x, y) =
            Linear::make_screen(Rectangle::new(Point::new(42., 84.), Size::new(200., 100.)));

        // forward x
        assert_relative_eq!(x.transform(-1.), 42.);
        assert_relative_eq!(x.transform(0.), 142.);
        assert_relative_eq!(x.transform(1.), 242.);

        // forward y
        assert_relative_eq!(y.transform(-1.), 184.);
        assert_relative_eq!(y.transform(0.), 134.);
        assert_relative_eq!(y.transform(1.), 84.);

        // inverse x
        assert_relative_eq!(x.inverse(42.), -1.);
        assert_relative_eq!(x.inverse(242.), 1.);

        // inverse y
        assert_relative_eq!(y.inverse(84.), 1.);
        assert_relative_eq!(y.inverse(184.), -1.);
    }

    #[test]
    fn instant_view() {
        let rate = SampleRate::new(1);
        let t = InstantView::new(Period::new(42, 100, rate));

        assert_relative_eq!(t.transform(Instant::new(32, rate)), -1.2);
        assert_relative_eq!(t.transform(Instant::new(42, rate)), -1.0);
        assert_relative_eq!(t.transform(Instant::new(92, rate)), 0.0);
        assert_relative_eq!(t.transform(Instant::new(142, rate)), 1.0);
        assert_relative_eq!(t.transform(Instant::new(152, rate)), 1.2);

        assert_eq!(t.inverse(-1.2), Instant::new(32, rate));
        assert_eq!(t.inverse(-1.0), Instant::new(42, rate));
        assert_eq!(t.inverse(1.0), Instant::new(142, rate));
    }
}
