//! # Format module
//! This module has functions to convert chords and notes to strings in different formats.
//!
//! ## European / default format
//! By default, `Chord` and `Note` implement `std::fmt::Display`.
//!
//! ## Latin
//! Do, Re, Mi, etc. notation.
//!
//! For notes, call `latin(&Note)`.
//! For chords, wrap them in `Latin`, which implements the `std::fmt::Display` trait.
//!

use crate::chords::{Chord, Note};
use std::fmt;
use std::fmt::{Display, Formatter};

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Note::A => "A",
                Note::ASharp => "Bb",
                Note::B => "B",
                Note::C => "C",
                Note::CSharp => "C#",
                Note::D => "D",
                Note::DSharp => "D#",
                Note::E => "E",
                Note::F => "F",
                Note::FSharp => "F#",
                Note::G => "G",
                Note::GSharp => "G#",
            }
        )
    }
}

impl std::fmt::Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)?;

        if self.minor {
            write!(f, "m")?;
        }

        write!(f, "{}", self.others)?;

        if self.number != 0 {
            write!(f, "{}", self.number)?;
        }

        if self.bass != self.root {
            write!(f, "/{}", self.bass)?;
        }

        Ok(())
    }
}

/// Shows the note in latin format (Do, Re, Mi...)
pub fn latin(n: &Note) -> impl Display {
    match n {
        Note::A => "La",
        Note::ASharp => "Sib",
        Note::B => "Si",
        Note::C => "Do",
        Note::CSharp => "Do#",
        Note::D => "Re",
        Note::DSharp => "Re#",
        Note::E => "Mi",
        Note::F => "Fa",
        Note::FSharp => "Fa#",
        Note::G => "Sol",
        Note::GSharp => "Sol#",
    }
}

/// Wrapper to format chord in latin format (Do, Re, Mi, Dom, Re7...)
pub struct Latin<'a>(pub &'a Chord);

impl<'a> Display for Latin<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", latin(&self.0.root))?;

        if self.0.minor {
            write!(f, "m")?;
        }

        if self.0.number != 0 {
            write!(f, "{}", self.0.number)?;
        }
        if !self.0.others.is_empty() {
            write!(f, "{}", self.0.others)?;
        }

        Ok(())
    }
}

impl<'a> AsRef<Chord> for Latin<'a> {
    fn as_ref(&self) -> &Chord {
        self.0
    }
}

#[cfg(test)]
mod test {
    use crate::{latin, Chord, Latin, Note};
    use std::string::ToString;

    #[test]
    fn standard_note() {
        assert_eq!(Note::A.to_string(), "A");
        assert_eq!(Note::ASharp.to_string(), "Bb");
        assert_eq!(Note::CSharp.to_string(), "C#");
    }

    #[test]
    fn standard_chord() {
        assert_eq!(Chord::major(Note::C).to_string(), "C");
        assert_eq!(Chord::major(Note::FSharp).to_string(), "F#");
        assert_eq!(Chord::minor(Note::E).to_string(), "Em");
        assert_eq!(Chord::minor(Note::GSharp).to_string(), "G#m");
    }

    #[test]
    fn latin_note() {
        assert_eq!(latin(&Note::B).to_string(), "Si");
        assert_eq!(latin(&Note::ASharp).to_string(), "Sib");
        assert_eq!(latin(&Note::DSharp).to_string(), "Re#");
    }

    #[test]
    fn latin_chord() {
        assert_eq!(Latin(&Chord::major(Note::D)).to_string(), "Re");
        assert_eq!(Latin(&Chord::minor(Note::A)).to_string(), "Lam");
        assert_eq!(Latin(&Chord::major(Note::ASharp)).to_string(), "Sib");
        assert_eq!(Latin(&Chord::minor(Note::GSharp)).to_string(), "Sol#m");
    }
}
