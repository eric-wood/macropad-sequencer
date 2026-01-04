mod render;
mod sequencer;
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
    value: u32,
    buffer: itoa::Buffer,
}

impl<'a> NumericMenuItem<'a> {
    pub fn new(title: &'a str, value: u32) -> Self {
        let buffer = itoa::Buffer::new();
        Self {
            title,
            value,
            buffer,
        }
    }

    pub fn inner(&self) -> &u32 {
        &self.value
    }
}

impl<'a> MenuItem for NumericMenuItem<'a> {
    fn as_str(&mut self) -> (&str, &str) {
        (self.title, self.buffer.format(self.value))
    }

    fn on_change(&mut self, step: i32) {
        let mut intermediate = self.value as i32;
        intermediate += step;
        if intermediate < 0 {
            intermediate = 0;
        }

        self.value = intermediate as u32;
    }
}

pub struct BooleanMenuItem<'a> {
    title: &'a str,
    value: bool,
    on_str: &'a str,
    off_str: &'a str,
}

impl<'a> BooleanMenuItem<'a> {
    pub fn new(title: &'a str, on_str: &'a str, off_str: &'a str) -> Self {
        Self {
            title,
            value: false,
            on_str,
            off_str,
        }
    }

    pub fn inner(&self) -> &bool {
        &self.value
    }
}

impl MenuItem for BooleanMenuItem<'static> {
    fn as_str(&mut self) -> (&str, &str) {
        let value = if self.value {
            self.on_str
        } else {
            self.off_str
        };
        (self.title, value)
    }

    fn on_change(&mut self, _step: i32) {
        self.value = !self.value;
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
}

impl<'a, const SIZE: usize, T> EnumMenuItem<'a, SIZE, T>
where
    T: Stringable,
{
    pub fn new(title: &'a str, options: [T; SIZE]) -> Self {
        Self {
            title,
            options,
            index: 0,
        }
    }

    pub fn inner(&self) -> &T {
        &self.options[self.index]
    }
}

impl<'a, const SIZE: usize, T> MenuItem for EnumMenuItem<'a, SIZE, T>
where
    T: Stringable + Copy,
{
    fn as_str(&mut self) -> (&str, &str) {
        (self.title, self.options[self.index].as_str())
    }

    fn on_change(&mut self, step: i32) {
        let next = (self.index as i32 + step).rem_euclid(SIZE as i32);
        self.index = next as usize;
    }
}
