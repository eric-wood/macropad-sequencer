mod render;
mod sequencer;
mod step;

pub use render::*;
pub use sequencer::{SEQUENCER_MENU, SequencerMenuItems, SequencerMenuValue};
pub use step::{Note, StepMenuItems, StepMenuValue};

use crate::display::MonoDisplay;

const WIDTH: u32 = 128;
const _HEIGHT: u32 = 64;

pub struct Menu<'a, T, const SIZE: usize>
where
    T: Copy,
{
    title: &'a str,
    index: usize,
    items: [&'a mut dyn MenuItem<T>; SIZE],
    selecting: bool,
    pub value: T,
    callback: &'a dyn Fn(&T),
}

impl<'a, T, const SIZE: usize> Menu<'a, T, SIZE>
where
    T: Copy,
{
    pub fn new(
        title: &'a str,
        value: T,
        items: [&'a mut dyn MenuItem<T>; SIZE],
        callback: &'a dyn Fn(&T),
    ) -> Self {
        let index = 0;
        let selecting = false;

        Self {
            title,
            index,
            items,
            selecting,
            value,
            callback,
        }
    }

    pub async fn on_change(&mut self, step: i32) {
        if self.selecting {
            let next = (self.index as i32 + step).rem_euclid(SIZE as i32);
            self.index = next as usize;
        } else {
            self.items[self.index].on_change(&mut self.value, step);
            (*self.callback)(&self.value);
        }
    }

    pub fn on_select(&mut self) {
        self.selecting = !self.selecting;
    }

    pub fn render(&mut self, display: &mut MonoDisplay) {
        render_menu_heading(display, self.title);

        for (i, item) in self.items.iter_mut().enumerate() {
            let (title, value) = item.as_str();

            let state = if i == self.index {
                if self.selecting {
                    MenuItemState::Selecting
                } else {
                    MenuItemState::Selected
                }
            } else {
                MenuItemState::None
            };

            render_menu_item(
                display,
                &MenuItemRender {
                    position: i,
                    title,
                    value,
                    state,
                },
            );
        }
    }
}

pub trait MenuItem<T> {
    fn as_str(&mut self) -> (&str, &str);
    fn on_change(&mut self, value: &mut T, step: i32);
}

pub struct NumericMenuItem<'a, T> {
    title: &'a str,
    pub value: u32,
    buffer: itoa::Buffer,
    callback: &'a dyn Fn(&mut T, u32),
}

impl<'a, T> NumericMenuItem<'a, T> {
    pub fn new(title: &'a str, value: u32, on_change: &'a dyn Fn(&mut T, u32)) -> Self {
        let buffer = itoa::Buffer::new();

        Self {
            title,
            value,
            buffer,
            callback: on_change,
        }
    }
}

impl<'a, T> MenuItem<T> for NumericMenuItem<'a, T> {
    fn as_str(&mut self) -> (&str, &str) {
        (self.title, self.buffer.format(self.value))
    }

    fn on_change(&mut self, value: &mut T, step: i32) {
        let mut intermediate = self.value as i32;
        intermediate += step;
        if intermediate < 0 {
            intermediate = 0;
        }

        self.value = intermediate as u32;
        (*self.callback)(value, self.value);
    }
}

pub struct BooleanMenuItem<'a, T> {
    title: &'a str,
    pub value: bool,
    on_str: &'a str,
    off_str: &'a str,
    callback: &'a dyn Fn(&mut T, bool),
}

impl<'a, T> BooleanMenuItem<'a, T> {
    pub fn new(
        title: &'a str,
        on_str: &'a str,
        off_str: &'a str,
        value: bool,
        on_change: &'a dyn Fn(&mut T, bool),
    ) -> Self {
        Self {
            title,
            value,
            on_str,
            off_str,
            callback: on_change,
        }
    }
}

impl<'a, T> MenuItem<T> for BooleanMenuItem<'a, T> {
    fn as_str(&mut self) -> (&str, &str) {
        let value = if self.value {
            self.on_str
        } else {
            self.off_str
        };
        (self.title, value)
    }

    fn on_change(&mut self, value: &mut T, _step: i32) {
        self.value = !self.value;
        (*self.callback)(value, self.value);
    }
}

pub trait Stringable {
    fn as_str(&self) -> &str;
}

pub struct EnumMenuItem<'a, T, const SIZE: usize, E>
where
    E: Stringable,
{
    title: &'a str,
    options: [E; SIZE],
    index: usize,
    pub value: E,
    callback: &'a dyn Fn(&mut T, E),
}

impl<'a, T, const SIZE: usize, E> EnumMenuItem<'a, T, SIZE, E>
where
    E: Stringable + PartialEq,
{
    pub fn new(
        title: &'a str,
        options: [E; SIZE],
        value: E,
        on_change: &'a dyn Fn(&mut T, E),
    ) -> Self {
        let index = options.iter().position(|i| *i == value).unwrap_or(0);

        Self {
            title,
            options,
            index,
            value,
            callback: on_change,
        }
    }

    pub fn set(&mut self, value: E) {
        self.index = self.options.iter().position(|i| *i == value).unwrap_or(0);
        self.value = value;
    }
}

impl<'a, T, const SIZE: usize, E> MenuItem<T> for EnumMenuItem<'a, T, SIZE, E>
where
    E: Stringable + Copy,
{
    fn as_str(&mut self) -> (&str, &str) {
        (self.title, self.options[self.index].as_str())
    }

    fn on_change(&mut self, value: &mut T, step: i32) {
        let next = (self.index as i32 + step).rem_euclid(SIZE as i32);
        self.index = next as usize;
        self.value = self.options[self.index];
        (*self.callback)(value, self.value);
    }
}
