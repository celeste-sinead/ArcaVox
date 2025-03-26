use iced::{mouse, widget::canvas};
use iced::{Color, Rectangle, Renderer, Theme};

pub struct Spectrogram {
    pub radius: f32,
}

impl<Message> canvas::Program<Message> for Spectrogram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        println!("bounds: {:?}", bounds);
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // We create a `Path` representing a simple circle
        let circle = canvas::Path::circle(frame.center(), self.radius);

        // And fill it with some color
        frame.fill(&circle, Color::BLACK);

        // Then, we produce the geometry
        vec![frame.into_geometry()]
    }
}
