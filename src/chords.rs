use num_traits::{FromPrimitive, ToPrimitive};
use serde::Serialize;

#[derive(Copy, Serialize, Debug, PartialEq, Clone, FromPrimitive, ToPrimitive)]
pub enum Note {
    A,
    ASharp,
    B,
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
}

#[derive(Serialize, Debug, Default, PartialEq, Clone)]
pub struct Chord {
    pub root: Note,
    pub minor: bool,
    pub others: String,
    pub number: u8,
    pub bass: Note,
}

impl Chord {
    pub fn major(n: Note) -> Self {
        Chord {
            root: n,
            minor: false,
            others: String::new(),
            number: 0,
            bass: n,
        }
    }

    pub fn minor(n: Note) -> Self {
        Chord {
            root: n,
            minor: true,
            others: String::new(),
            number: 0,
            bass: n,
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

impl Default for Note {
    fn default() -> Self {
        Note::A
    }
}
