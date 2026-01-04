use crate::{
    display::MonoDisplay,
    menus::{
        Menu, MenuItem, MenuItemRender, MenuItemState, Stringable, render_menu_heading,
        render_menu_item,
    },
    sequencer_timer::TimingOption,
};

impl Stringable for TimingOption {
    fn as_str(&self) -> &str {
        match self {
            TimingOption::Quarter => "1/4",
            TimingOption::QuarterTriplet => "1/4 triplet",
            TimingOption::Eighth => "1/8",
            TimingOption::EighthTriplet => "1/8 triplet",
            TimingOption::Sixteenth => "1/16",
            TimingOption::SixteenthTriplet => "1/16 triplet",
        }
    }
}

pub struct SequencerMenu<'a, const SIZE: usize> {
    index: usize,
    items: [&'a mut dyn MenuItem; SIZE],
    selecting: bool,
}

impl<'a, const SIZE: usize> SequencerMenu<'a, SIZE> {
    pub fn new(items: [&'a mut dyn MenuItem; SIZE]) -> Self {
        let index = 0;
        let selecting = false;

        Self {
            index,
            items,
            selecting,
        }
    }
}

impl<'a, const SIZE: usize> Menu for SequencerMenu<'a, SIZE> {
    fn title(&self) -> &str {
        "Sequencer"
    }

    fn on_change(&mut self, step: i32) {
        if self.selecting {
            let next = (self.index as i32 + step).rem_euclid(SIZE as i32);
            self.index = next as usize;
        } else {
            self.items[self.index].on_change(step);
        }
    }

    fn on_select(&mut self) {
        self.selecting = !self.selecting;
    }

    fn render(&mut self, display: &mut MonoDisplay) {
        render_menu_heading(display, self.title());

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
