use crate::menus::{EnumMenuItem, NumericMenuItem, Stringable};

#[derive(Clone, Copy, PartialEq)]
pub enum Note {
    A,
    BFlat,
    B,
    C,
    CSharp,
    D,
    EFlat,
    E,
    F,
    FSharp,
    G,
    AFlat,
}

impl Stringable for Note {
    fn as_str(&self) -> &str {
        match self {
            Note::A => "A",
            Note::BFlat => "Bb",
            Note::B => "B",
            Note::C => "C",
            Note::CSharp => "C#",
            Note::D => "D",
            Note::EFlat => "Eb",
            Note::E => "E",
            Note::F => "F",
            Note::FSharp => "F#",
            Note::G => "G",
            Note::AFlat => "Ab",
        }
    }
}

#[derive(Clone, Copy)]
pub struct StepMenuValue {
    pub note: Note,
    pub octave: u32,
    pub velocity: u32,
}

impl Default for StepMenuValue {
    fn default() -> Self {
        Self {
            note: Note::C,
            octave: 1,
            velocity: 100,
        }
    }
}

pub struct StepMenuItems<'a> {
    pub note_menu: EnumMenuItem<'a, StepMenuValue, 12, Note>,
    pub octave_menu: NumericMenuItem<'a, StepMenuValue>,
    pub velocity_menu: NumericMenuItem<'a, StepMenuValue>,
}

impl<'a> StepMenuItems<'a> {
    pub fn new() -> Self {
        let defaults = StepMenuValue::default();

        let note_menu = EnumMenuItem::<'_, StepMenuValue, 12, Note>::new(
            "NOTE",
            [
                Note::A,
                Note::BFlat,
                Note::B,
                Note::C,
                Note::CSharp,
                Note::D,
                Note::EFlat,
                Note::E,
                Note::F,
                Note::FSharp,
                Note::G,
                Note::AFlat,
            ],
            defaults.note,
            &|menu_value, value| {
                menu_value.note = value;
            },
        );

        let octave_menu = NumericMenuItem::<StepMenuValue>::new(
            "OCTAVE",
            defaults.octave,
            &|menu_value, value| {
                menu_value.octave = value;
            },
        );

        let velocity_menu = NumericMenuItem::<StepMenuValue>::new(
            "VELOCITY",
            defaults.velocity,
            &|menu_value, value| {
                menu_value.velocity = value;
            },
        );

        Self {
            note_menu,
            octave_menu,
            velocity_menu,
        }
    }
}
