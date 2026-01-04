use embedded_graphics::{
    mono_font::{
        MonoTextStyle, MonoTextStyleBuilder,
        ascii::{FONT_5X8, FONT_6X13},
    },
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StyledDrawable},
    text::{Alignment, Baseline, Text, TextStyle, TextStyleBuilder, renderer::TextRenderer},
};

use crate::{display::MonoDisplay, menus::WIDTH};

#[derive(PartialEq, Eq, Debug)]
pub enum MenuItemState {
    None,
    Selecting,
    Selected,
}

#[derive(PartialEq, Eq)]
pub struct MenuItemRender<'a> {
    pub position: usize,
    pub title: &'a str,
    pub value: &'a str,
    pub state: MenuItemState,
}

const TITLE_CHAR_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyleBuilder::new()
    .font(&FONT_5X8)
    .text_color(BinaryColor::On)
    .build();

const TITLE_TEXT_STYLE: TextStyle = TextStyleBuilder::new()
    .alignment(Alignment::Center)
    .baseline(Baseline::Top)
    .build();

const TEXT_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyleBuilder::new()
    .font(&FONT_6X13)
    .text_color(BinaryColor::On)
    .build();

const SELECTED_TEXT_STYLE: MonoTextStyle<'_, BinaryColor> = MonoTextStyleBuilder::new()
    .font(&FONT_6X13)
    .text_color(BinaryColor::Off)
    .build();

const RECTANGLE_STYLE: PrimitiveStyle<BinaryColor> = PrimitiveStyle::with_fill(BinaryColor::On);

pub fn render_menu_heading(display: &mut MonoDisplay, title: &str) {
    Text::with_text_style(
        title,
        Point::new(WIDTH as i32 / 2, 0),
        TITLE_CHAR_STYLE,
        TITLE_TEXT_STYLE,
    )
    .draw(display)
    .unwrap();

    let _ = Line::new(Point::new(0, 9), Point::new(WIDTH as i32, 9)).draw_styled(
        &PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .stroke_width(1)
            .build(),
        display,
    );
}

pub fn render_menu_item(display: &mut MonoDisplay, item: &MenuItemRender) {
    let padding = 2;
    let offset_top = 11;
    let height = TEXT_STYLE.line_height() as i32;
    let width = FONT_6X13.character_size.width as i32;
    // account for the fact that glyphs can be shorter than the line height
    let offset = -4;
    let section_height = height + padding * 2 + offset;
    let position = item.position as i32;

    let title_position = Point::new(padding, section_height * position + offset_top);
    let value_position = Point::new(
        title_position.x + width * ((item.title.len() + 1) as i32),
        title_position.y,
    );
    let title_style = if item.state == MenuItemState::Selecting {
        SELECTED_TEXT_STYLE
    } else {
        TEXT_STYLE
    };
    let value_style =
        if item.state == MenuItemState::Selecting || item.state == MenuItemState::Selected {
            SELECTED_TEXT_STYLE
        } else {
            TEXT_STYLE
        };

    if item.state == MenuItemState::Selecting {
        Rectangle::new(
            Point::new(0, title_position.y),
            Size::new(WIDTH, section_height as u32),
        )
        .draw_styled(&RECTANGLE_STYLE, display)
        .unwrap();
    }

    if item.state == MenuItemState::Selected {
        Rectangle::new(
            Point::new(value_position.x - padding, value_position.y),
            Size::new(
                ((item.value.len() as i32) * width + padding * 2 - 1) as u32,
                section_height as u32,
            ),
        )
        .draw_styled(&RECTANGLE_STYLE, display)
        .unwrap();
    }

    Text::with_baseline(item.title, title_position, title_style, Baseline::Top)
        .draw(display)
        .unwrap();
    Text::with_baseline(item.value, value_position, value_style, Baseline::Top)
        .draw(display)
        .unwrap();
}
