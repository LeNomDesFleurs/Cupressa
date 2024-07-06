// mod audio;
// mod midi;
// mod parameters;
// mod synth;
// mod ui;
// use parameters::ParameterID;
// use parameters::Parameters;
// pub use synth::Synth;
// mod oscillator;
// pub use oscillator::HarmonicOscillator;
// pub use oscillator::Lfo;
// mod filter;
// pub use filter::Biquad;
// mod buffer;
// pub use buffer::RingBuffer;
// mod chorus;
// mod outils;
// pub use chorus::Chorus;
// mod textparsing;
// pub use textparsing::TextCharacteristic;
// mod envelope;
// mod midibuffer;
// mod reverb;
/* This example expose parameter to pass generator of sample.
Good starting point for integration of cpal into your application.
*/
// type ParameterUpdate = (ParameterID, f32);
use std::error::Error;
extern crate anyhow;
// use crate::midi::MidiMessage;
use crossterm::terminal::disable_raw_mode;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::io;


pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command,
};

fn main() -> Result<(), Box<dyn Error>> {
    // disable_raw_mode().unwrap();

    // get terminal size
    // deform space
    let default_terminal_size = (10 as u16, 10 as u16);

    let terminal_size = crossterm::terminal::size().unwrap_or(default_terminal_size);
    let mut stdout = io::stdout();
    for i in 0..terminal_size.0 {
        for j in 0..terminal_size.1{
        execute!(stdout,
        cursor::MoveTo(i, j),
        style::Print("?"),
    )?;
    thread::sleep(time::Duration::from_millis(500));
    }}

    // use std::panic;

    Ok(())
}
