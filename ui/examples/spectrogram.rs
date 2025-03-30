use audio::stream::buffer::SampleBuffer;
use audio::synth::ChirpIterator;
use iced::widget::canvas;
use iced::{widget, Element, Length, Padding};

use audio::dsp::fft::{FFTSequence, FoldedFFT};
use audio::stream::{ChannelCount, Period, SampleRate};

extern crate ui;
use ui::spectrogram::Spectrogram;

/// FFT window size / spectrogram vertical resolution is half this:
const WINDOW_SIZE: usize = 64;
/// Number of windows / spectrogram horizontal resolution:
const WINDOW_COUNT: usize = 128;
/// Initial frequency of the chirp signal (Hz):
const BASE_FREQ: f32 = 0.0;
/// Frequency slope of the (linear) chirp (Hz):
const FREQ_SLOPE: f32 = 0.1;
/// Kind of an arbitrary scalar, given the current absence of axis labels:
const SAMPLE_RATE: u32 = 32;

struct SpecExample {
    ffts: Vec<FoldedFFT>,
}

impl Default for SpecExample {
    fn default() -> Self {
        let sample_rate = SampleRate::new(SAMPLE_RATE);
        let mut synth = ChirpIterator::new(sample_rate, BASE_FREQ, FREQ_SLOPE);
        let mut buf = SampleBuffer::new(ChannelCount::new(1), sample_rate, WINDOW_SIZE);
        let ffter = FFTSequence::new(WINDOW_SIZE);
        let mut ffts = Vec::new();

        for i in 0..WINDOW_COUNT {
            buf.push_some_mono(&mut synth, WINDOW_SIZE);
            let window = Period::new(i * WINDOW_SIZE, WINDOW_SIZE, sample_rate);
            ffts.push(
                ffter
                    .fft(&buf.get_window(window).get_channel(0))
                    .into_polar()
                    .into_folded(),
            );
        }

        SpecExample { ffts }
    }
}

#[derive(Debug)]
struct Message();

fn update(_ex: &mut SpecExample, _message: Message) {}

fn view(ex: &SpecExample) -> Element<Message> {
    widget::Container::new(
        canvas(Spectrogram::new(ex.ffts.clone()))
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(Padding::new(5.))
    .into()
}

fn main() -> iced::Result {
    iced::application("Spectrogram Example", update, view).run()
}
