//! Tools for prototyping in a Jupyter notebook
//! The idea is that a notebook can just import this and be unlikely to need
//! anything else, i.e. start with:
//! ```ipynb
//! :dep notebook = { path = "." }
//! use notebook::*;
//! ```

pub use std::f32::consts::PI;

pub use audio;
pub use audio::dsp::fft::FoldedFFT;
pub use audio::stream::Duration;
pub use audio::stream::buffer::{BufferedInput, Period};
pub use audio::stream::input::SampleRate;
pub use charts;
pub use num_complex::Complex;
pub use plotters;
use plotters::evcxr::SVGWrapper;
pub use plotters::prelude::*;

pub fn plot_period(period: &Period) -> SVGWrapper {
    evcxr_figure((640, 480), |root| {
        assert!(u16::from(period.channel_count()) == 1);

        root.fill(&WHITE)?;
        let start_s = Duration::from_start(period.start_time()).as_secs_f32();
        let end_s = Duration::from_start(period.end_time()).as_secs_f32();
        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d( start_s..end_s, -1f32..1f32,)?;
        chart.configure_mesh().draw()?;

        let series = period
            .get_channel(0)
            .into_timeseries()
            .map(|(t, y)| (t.as_secs_from_start_f32(), y));

        chart.draw_series(LineSeries::new(series, &RED)).unwrap();

        Ok(())
    })
}

pub fn plot_fft(fft: &FoldedFFT) -> SVGWrapper {
    evcxr_figure((640, 480), |root| {
        root.fill(&WHITE)?;
        charts::build_fft_chart(ChartBuilder::on(&root), fft)?;
        Ok(())
    })
}
