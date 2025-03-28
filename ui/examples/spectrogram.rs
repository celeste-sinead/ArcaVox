use iced::widget::canvas;
use iced::{widget, Element, Length, Padding};

use audio::dsp::fft::FoldedFFT;
use audio::stream::SampleRate;

extern crate ui;
use ui::spectrogram::Spectrogram;

#[derive(Default)]
struct SpecExample();

#[derive(Debug)]
struct Message();

fn update(_ex: &mut SpecExample, _message: Message) {}

fn view(_ex: &SpecExample) -> Element<Message> {
    let fft = FoldedFFT::from_magnitudes(&[1.0, 0.0, 0.5, 0.25], SampleRate::new(8));
    widget::Container::new(widget::column![
        widget::text("Hello Iced!"),
        canvas(Spectrogram::new(fft))
            .width(Length::Fill)
            .height(Length::Fill)
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(Padding::new(5.))
    .into()
}

fn main() -> iced::Result {
    iced::application("Spectrogram Example", update, view).run()
}
