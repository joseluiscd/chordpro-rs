//! # Chordpro
//! This crate is a chordpro parser. Chordpro is a simple text format for
//! the notation of lyrics with chords. Although initially intended for
//! guitarists, it can be used for all kinds of musical purposes.
//! Specification of the format can be found in the official website:
//! [https://www.chordpro.org/](https://www.chordpro.org/)
//! 
//! To build a `Song` from a chordpro file:
//! ```
//! # use chordpro::Song;
//! # use std::str::FromStr;
//! 
//! let song = Song::from_str(r##"
//!     {title: Song Title}"
//!     {artist: The Artist}
//!     
//!     This is the first verse.
//!     You can specify chords using brackets.
//!     This is a [G]chord
//! 
//!     {soc}
//!     This is the chorus of the song
//!     [Em]You can also add some chords
//!     {eoc}
//! "##).unwrap();
//! ```

#![crate_name = "chordpro"]

extern crate pest;
#[macro_use] extern crate pest_derive;
extern crate serde;
extern crate num_traits;
#[macro_use] extern crate num_derive;

pub mod chords;
pub mod song;
pub mod iterators;
pub mod songparse;
pub mod transpose;
pub mod format;

pub use {
    chords::{
        Chord,
        Note
    },
    song::{
        Song,
        Section,
        Paragraph,
        Line,
        Chunk
    },
    iterators::{
        SectionIterator,
        SectionMutIterator,
    },
    format::{
        latin, Latin
    }
};