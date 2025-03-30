use iced::{mouse, widget::canvas};
use iced::{Color, Point, Rectangle, Renderer, Size, Theme};

use audio::dsp::fft::FoldedFFT;

pub struct Spectrogram {
    ffts: Vec<FoldedFFT>,
}

impl Spectrogram {
    #[must_use]
    pub fn new(ffts: Vec<FoldedFFT>) -> Self {
        Self { ffts }
    }
}

impl<Message> canvas::Program<Message> for Spectrogram {
    type State = ();

    #[allow(clippy::cast_precision_loss)]
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        // Render each FFT along the vertical axis, i.e. the vertical axis is frequency (increasing
        // upward) and the horizontal axis is time (increasing rightward)
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        // First check if there are any ffts; render nothing if not
        if let Some(first) = self.ffts.first() {
            // (Assuming constant FFT size) compute the fraction of the frame that each frequency
            // bin should fill in order to completely tile the frame with bins.
            let bin_width_frac: f32 = 1. / self.ffts.len() as f32;
            let bin_height_frac = 1. / first.values.len() as f32;
            // for each FFT / column:
            for (i, fft) in self.ffts.iter().enumerate() {
                // for each frequency bin / row: (r=magnitude, θ=phase)
                for (j, (r, _θ)) in fft.values.iter().enumerate() {
                    let top_left = Point::new(
                        (i as f32) * bin_width_frac * frame.width(),
                        (1.0 - (j as f32 + 1.) * bin_height_frac) * frame.height(),
                    );
                    frame.fill_rectangle(
                        top_left,
                        Size::new(
                            frame.width() * bin_width_frac,
                            frame.height() * bin_height_frac,
                        ),
                        Color::from_rgb(*r, 0., *r),
                    );
                }
            }
        }

        vec![frame.into_geometry()]
    }
}
