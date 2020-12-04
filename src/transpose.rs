//! This module allows transposing chords in a `Song`
//!
use crate::song::{Chunk, Song};

use crate::chords::Chord;

pub struct Transposer {
    s: i8,
}

pub fn map_to_chords<'a, F>(song: &mut Song, f: F)
where
    F: Fn(&mut Chord),
{
    for section in song.iter_mut() {
        for line in section.iter_mut() {
            for chunk in line.iter_mut() {
                match chunk {
                    Chunk::Chord(c) => f(c),
                    _ => {}
                }
            }
        }
    }
}

impl Transposer {
    /// Create a `Transposer` object which transposes song the specified
    /// amount of `semitones` (positive or negative)
    pub fn new(semitones: i8) -> Self {
        Self { s: semitones }
    }

    /// Applies transposition in-place
    pub fn apply_transpose(&self, song: &mut Song) {
        if self.s.abs() % 12 != 0 {
            map_to_chords(song, |chord| {
                chord.root = chord.root + self.s;
                chord.bass = chord.bass + self.s;
            });
        }
    }

    /// Applies transposition to song
    pub fn transpose(&self, song: Song) -> Song {
        let mut song = song;
        self.apply_transpose(&mut song);
        song
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn transpose() {
        use super::*;
        use crate::chords::Note;
        use crate::song::{Line, Song};
        use std::str::FromStr;
        let song = Song::from_str("[C]A [D#m]Cm").expect("Failed to parse song");
        let song2 = Transposer::new(-3).transpose(song);

        let line = song2.iter().nth(0).unwrap().iter().nth(0).unwrap();
        assert_eq!(
            line,
            &Line(vec![
                Chunk::Chord(Chord::major(Note::A)),
                Chunk::Lyrics("A ".to_string()),
                Chunk::Chord(Chord::minor(Note::C)),
                Chunk::Lyrics("Cm".to_string()),
            ])
        );
    }
}
