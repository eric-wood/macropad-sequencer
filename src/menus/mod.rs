mod render;
mod sequencer;
mod step;

pub use render::*;
pub use sequencer::{SEQUENCER_MENU, SequencerMenuItems, SequencerMenuValue};

use crate::display::MonoDisplay;

const WIDTH: u32 = 128;
const HEIGHT: u32 = 64;

pub struct Menu<'a, const SIZE: usize> {
    title: &'a str,
    index: usize,
    items: [&'a mut dyn MenuItem; SIZE],
    selecting: bool,
}

impl<'a, const SIZE: usize> Menu<'a, SIZE> {
    pub fn new(title: &'a str, items: [&'a mut dyn MenuItem; SIZE]) -> Self {
        let index = 0;
        let selecting = false;

        Self {
            title,
            index,
            items,
            selecting,
        }
    }

    pub fn on_change(&mut self, step: i32) {
        if self.selecting {
            let next = (self.index as i32 + step).rem_euclid(SIZE as i32);
            self.index = next as usize;
        } else {
            self.items[self.index].on_change(step);
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

pub trait MenuItem {
    fn as_str(&mut self) -> (&str, &str);
    fn on_change(&mut self, step: i32);
}

pub struct NumericMenuItem<'a> {
    title: &'a str,
    value: u32,
    buffer: itoa::Buffer,
    callback: &'a dyn Fn(u32),
}

impl<'a> NumericMenuItem<'a> {
    pub fn new(title: &'a str, value: u32, on_change: &'a dyn Fn(u32)) -> Self {
        let buffer = itoa::Buffer::new();

        Self {
            title,
            value,
            buffer,
            callback: on_change,
        }
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
        (*self.callback)(self.value);
    }
}

pub struct BooleanMenuItem<'a> {
    title: &'a str,
    value: bool,
    on_str: &'a str,
    off_str: &'a str,
    callback: &'a dyn Fn(bool),
}

impl<'a> BooleanMenuItem<'a> {
    pub fn new(
        title: &'a str,
        on_str: &'a str,
        off_str: &'a str,
        value: bool,
        on_change: &'a dyn Fn(bool),
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
        (*self.callback)(self.value);
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
    value: T,
    callback: &'a dyn Fn(T),
}

impl<'a, const SIZE: usize, T> EnumMenuItem<'a, SIZE, T>
where
    T: Stringable,
{
    pub fn new(title: &'a str, options: [T; SIZE], value: T, on_change: &'a dyn Fn(T)) -> Self {
        // TODO: set index to currently selected!

        Self {
            title,
            options,
            index: 0,
            value,
            callback: on_change,
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
        self.value = self.options[self.index];
        (*self.callback)(self.value);
    }
}
