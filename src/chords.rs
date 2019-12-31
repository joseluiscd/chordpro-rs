use serde::Serialize;
use num_traits::{
    FromPrimitive,
    ToPrimitive
};


#[derive(Copy, Serialize, Debug, PartialEq, Clone, FromPrimitive, ToPrimitive)]
pub enum Note {
    A, ASharp, B, C, CSharp, D, DSharp, E, F, FSharp, G, GSharp
}

#[derive(Serialize, Debug, Default, PartialEq, Clone)]
pub struct Chord{
    pub root: Note,
    pub minor: bool,
    pub others: String,
    pub number: u8,
    pub bass: Note
}

impl Chord {
    pub fn major(n: Note) -> Self {
        Chord {
            root: n,
            minor: false,
            others: String::new(),
            number: 0,
            bass: n
        }
    }

    pub fn minor(n: Note) -> Self {
        Chord {
            root: n,
            minor: true,
            others: String::new(),
            number: 0,
            bass: n
        }
    }
}

impl std::ops::Add<i8> for Note {
    type Output = Note;
    fn add(self, other: i8) -> Self::Output {
        let k = (self.to_i8().unwrap() + other + 12) % 12;

        Note::from_i8(k).unwrap()
    }
}

impl std::ops::Sub<i8> for Note {
    type Output = Note;
    fn sub(self, other: i8) -> Self::Output {
        self + (-other)
    }
}

impl Default for Note{
    fn default() -> Self {
        Note::A
    }
}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate(){
            write!(f, "{}", match self {
                Note::A => "A",
                Note::ASharp => "A#",
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
            })
        } else{
            write!(f, "{}", match self {
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
            })
        }
    }
}

impl std::fmt::Binary for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate(){
            write!(f, "{}", match self {
                Note::A => "La",
                Note::ASharp => "La#",
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
            })
        } else{
            write!(f, "{}", match self {
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
            })
        }
    }
}

impl std::fmt::Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate(){
            write!(f, "{:#}", self.root)
        } else {
            write!(f, "{}", self.root)
        }?;

        if self.minor {
            write!(f, "m")?;
        }

        write!(f, "{}", self.others)
    }
}

impl std::fmt::Binary for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate(){
            write!(f, "{:#b}", self.root)
        } else {
            write!(f, "{:b}", self.root)
        }?;

        if self.minor {
            write!(f, "m")?;
        }

        write!(f, "{}", self.others)
    }
}