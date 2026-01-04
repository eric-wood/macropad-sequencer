mod render;
mod sequencer;
use core::sync::atomic::{AtomicBool, AtomicU8, AtomicU32, Ordering};

pub use render::*;
pub use sequencer::SequencerMenu;

use crate::display::MonoDisplay;

const WIDTH: u32 = 128;
const HEIGHT: u32 = 64;

pub trait Menu {
    fn title(&self) -> &str;
    fn on_change(&mut self, step: i32);
    fn on_select(&mut self);
    fn render(&mut self, display: &mut MonoDisplay);
}

pub trait MenuItem {
    fn as_str(&mut self) -> (&str, &str);
    fn on_change(&mut self, step: i32);
}

pub struct NumericMenuItem<'a> {
    title: &'a str,
    value: &'a AtomicU32,
    buffer: itoa::Buffer,
}

impl<'a> NumericMenuItem<'a> {
    pub fn new(title: &'a str, value: &'a AtomicU32) -> Self {
        let buffer = itoa::Buffer::new();

        Self {
            title,
            value,
            buffer,
        }
    }
}

impl<'a> MenuItem for NumericMenuItem<'a> {
    fn as_str(&mut self) -> (&str, &str) {
        (
            self.title,
            self.buffer.format(self.value.load(Ordering::Relaxed)),
        )
    }

    fn on_change(&mut self, step: i32) {
        let mut intermediate = self.value.load(Ordering::Relaxed) as i32;
        intermediate += step;
        if intermediate < 0 {
            intermediate = 0;
        }

        self.value.store(intermediate as u32, Ordering::Relaxed);
    }
}

pub struct BooleanMenuItem<'a> {
    title: &'a str,
    value: &'a AtomicBool,
    on_str: &'a str,
    off_str: &'a str,
}

impl<'a> BooleanMenuItem<'a> {
    pub fn new(title: &'a str, on_str: &'a str, off_str: &'a str, value: &'a AtomicBool) -> Self {
        Self {
            title,
            value,
            on_str,
            off_str,
        }
    }
}

impl MenuItem for BooleanMenuItem<'static> {
    fn as_str(&mut self) -> (&str, &str) {
        let value = if self.value.load(Ordering::Relaxed) {
            self.on_str
        } else {
            self.off_str
        };
        (self.title, value)
    }

    fn on_change(&mut self, _step: i32) {
        self.value
            .store(!self.value.load(Ordering::Relaxed), Ordering::Relaxed);
    }
}

pub trait Stringable {
    fn as_str(&self) -> &str;
}

pub struct EnumMenuItem<'a, const SIZE: usize, T>
where
    T: Stringable,
{
    title: &'a str,
    options: [T; SIZE],
    index: usize,
    value: &'a AtomicU8,
}

impl<'a, const SIZE: usize, T> EnumMenuItem<'a, SIZE, T>
where
    T: Stringable,
{
    pub fn new(title: &'a str, options: [T; SIZE], value: &'a AtomicU8) -> Self {
        // TODO: set index to currently selected!

        Self {
            title,
            options,
            index: 0,
            value,
        }
    }
}

impl<'a, const SIZE: usize, T> MenuItem for EnumMenuItem<'a, SIZE, T>
where
    T: Stringable + Copy + Into<u8>,
{
    fn as_str(&mut self) -> (&str, &str) {
        (self.title, self.options[self.index].as_str())
    }

    fn on_change(&mut self, step: i32) {
        let next = (self.index as i32 + step).rem_euclid(SIZE as i32);
        self.index = next as usize;
        self.value
            .store(self.options[self.index].into(), Ordering::Relaxed);
    }
}
