//! Contains `Song` struct and its components.
//!
use crate::chords::Chord;
use serde::{Serialize, Deserialize};

/// Chunk of lyrics or a chord
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "class", content = "content")]
#[non_exhaustive]
pub enum Chunk {
    /// Some lyrics
    Lyrics(String),
    /// A chord represented by a &str
    Chord(Chord),
}

/// Lyrics with chords
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Line(pub Vec<Chunk>);

/// A verse/chorus in the song
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Paragraph(pub Vec<Line>);

/// A song section (chorus, verse or a comment)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "class", content = "content")]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum Section {
    /// The chorus
    Chorus(Paragraph),

    /// A verse
    Verse(Paragraph),

    /// Chordpro comment.
    /// Only contains one Line.
    Comment(Line),
}

impl Default for Section {
    fn default() -> Self {
        Section::Comment(Line::default())
    }
}

/// A song with its chords
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
#[non_exhaustive]
pub struct Song {
    pub title: String,
    pub subtitle: String,
    pub artist: String,
    pub capo: u8,
    pub song: Vec<Section>,
}
