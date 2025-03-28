use iced::{mouse, widget::canvas};
use iced::{Color, Point, Rectangle, Renderer, Size, Theme};

use audio::dsp::fft::FoldedFFT;

pub struct Spectrogram {
    fft: FoldedFFT
}

impl Spectrogram {
    #[must_use]
    pub fn new(fft: FoldedFFT) -> Self {
        Self { fft }
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
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let bin_frac = 1. /  self.fft.values.len() as f32;
        for (i, (r, _theta)) in self.fft.values.iter().enumerate() {
            frame.fill_rectangle(
                Point::new(0., (1.0 - (i as f32 + 1.)*bin_frac)*frame.height()),
                Size::new(frame.width()*bin_frac, frame.height()*bin_frac),
                Color::from_rgb(*r, *r, *r)
            );
        }

        vec![frame.into_geometry()]
    }
}
