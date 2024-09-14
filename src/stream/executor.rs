use std::thread;

use async_channel::{Receiver, Sender};

use super::buffer::{InputBuffer, PeriodStream};
use super::input::{ChannelCount, Frame, InputStream, SampleRate};
use super::wav::WavWriter;
use crate::dsp;
use crate::Message;

// The maximum length of channels passing audio data amongst threads
// This shouldn't be large; if a consumer isn't keeping up long channels are
// just going to add latency to the situation.
pub const CHANNEL_MAX: usize = 16;

pub struct Executor {
    channels: ChannelCount,
    sample_rate: SampleRate,
    writer: WavWriter,
    periods: PeriodStream,
    sender: Sender<Message>,
}

impl Executor {
    pub fn new(
        sender: Sender<Message>,
        channels: ChannelCount,
        sample_rate: SampleRate,
    ) -> Executor {
        Executor {
            channels,
            sample_rate,
            writer: WavWriter::new(channels, sample_rate),
            periods: PeriodStream::new(
                InputBuffer::new(channels, sample_rate, usize::from(sample_rate) * 2),
                usize::from(sample_rate) / 10,
                usize::from(sample_rate) / 10,
            ),
            sender,
        }
    }

    fn process(&mut self, frame: &Frame) -> Vec<Message> {
        let mut res = Vec::new();
        self.writer.push(frame).expect("session.wav write error");
        self.periods.push(frame);
        while let Some(p) = self.periods.next() {
            res.push(Message::RMSLevels {
                time: p.start_time(),
                values: p.channels().into_iter().map(|c| dsp::rms(&c)).collect(),
            });
        }
        res
    }

    fn run(mut self, frames: Receiver<Frame>) {
        loop {
            match frames.recv_blocking() {
                Ok(f) => {
                    for m in self.process(&f) {
                        if let Err(_) = self.sender.send_blocking(m) {
                            println!("Executor exit: UI closed.");
                            return;
                        }
                    }
                }
                Err(_) => {
                    println!("Executor exit: audio input closed.");
                    let _e = self.sender.send_blocking(Message::AudioStreamClosed);
                    return;
                }
            }
        }
    }

    pub fn start(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            // cpal::StreamTrait isn't Send, so the input device needs to
            // be opened on the executor thread.
            let input = InputStream::new(self.channels, self.sample_rate);
            self.run(input.frames);
        })
    }
}
