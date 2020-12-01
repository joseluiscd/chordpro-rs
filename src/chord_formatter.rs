//! # Formatting Chords
//!
use crate::chords::{Chord, Note};
use std::fmt::{
    Formatter, Display
};
use std::fmt;

pub trait ChordFormatter {
    fn format(&self, fmt: &mut Formatter, chord: &Chord) -> fmt::Result;
    fn format_note(&self, fmt: &mut Formatter, note: &Note) -> fmt::Result;
}


struct StandardFormatter;

impl ChordFormatter for StandardFormatter {
    fn format(&self, fmt: &mut Formatter, chord: &Chord) -> fmt::Result {
        Display::fmt(chord, fmt)
    }

    fn format_note(&self, fmt: &mut Formatter, note: &Note) -> fmt::Result {
        Display::fmt(note, fmt)
    }
}
