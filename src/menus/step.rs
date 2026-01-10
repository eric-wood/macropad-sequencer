use crate::{
    menus::{BooleanMenuItem, EnumMenuItem, NumericMenuItem, Stringable},
    sequencer_timer::TimingOption,
};

enum Note {
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

//pub struct StepMenuItems<'a> {
//    pub note_menu: EnumMenuItem<'a, 12>,
//    pub octave_menu: NumericMenuItem<'a>,
//    pub velocity_menu: NumericMenuItem<'a>,
//}
//
//impl<'a> StepMenuItems<'a> {
//    pub fn new() -> Self {
//
//        let note_menu = EnumMenuItem::new("NOTE", &[
//            Note::BFlat
//            Note::B,
//            Note::C,
//            Note::CSharp,
//            Note::D,
//            Note::EFlat,
//            Note::E,
//            Note::F,
//            Note::FSharp,
//            Note::G,
//            Note::AFlat,
//        ]);
//
//        let octave_menu = NumericMenuItem::new()
//
//        Self {
//            note_menu,
//        }
//    }
//}
