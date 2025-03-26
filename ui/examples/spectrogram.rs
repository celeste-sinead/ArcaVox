use iced::widget::canvas;
use iced::{widget, Element, Length, Padding, Size};

extern crate ui;
use ui::spectrogram::Spectrogram;

struct SpecExample();

impl Default for SpecExample {
    fn default() -> SpecExample {
        SpecExample()
    }
}

#[derive(Debug)]
struct Message();

fn update(_ex: &mut SpecExample, _message: Message) {}

fn view(_ex: &SpecExample) -> Element<Message> {
    widget::Container::new(widget::column![
        widget::text("Hello Iced!"),
        canvas(Spectrogram { radius: 100.0 })
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
