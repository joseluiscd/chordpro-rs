//! Contains `Song` struct and its components.
//! 
use serde::Serialize;
use crate::chords::Chord;


/// Chunk of lyrics or a chord
#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(tag = "class", content="content")]
pub enum Chunk {
    /// Some lyrics
    Lyrics(String),
    /// A chord represented by a &str
    Chord(Chord)
}

/// Lyrics with chords
#[derive(Serialize, Debug, Default, PartialEq, Clone)]
pub struct Line(pub Vec<Chunk>);

/// A verse/chorus in the song
#[derive(Serialize, Debug, Default, PartialEq, Clone)]
pub struct Paragraph(pub Vec<Line>);


/// A song section (chorus, verse or a comment)
#[derive(Serialize, Debug, PartialEq, Clone)]
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

/// A song with its chords
#[derive(Serialize, Debug, Default, PartialEq, Clone)]
#[non_exhaustive]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub capo: u8,
    pub song: Vec<Section>,
}