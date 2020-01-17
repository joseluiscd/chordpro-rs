# chordpro-rs

[![Latest Version](https://img.shields.io/crates/v/chordpro.svg)](https://crates.io/crates/chordpro)
[![Documentation](https://docs.rs/chordpro/badge.svg)](https://docs.rs/chordpro)

Chordpro file parser written in Rust. Chordpro is a simple text format for
the notation of lyrics with chords. Although initially intended for
guitarists, it can be used for all kinds of musical purposes.

Specification of the format can be found in the official website:
[https://www.chordpro.org/](https://www.chordpro.org/)

## Example
The `std::str::FromStr` is implemented for the `Song` struct:

```rust
# use chordpro::Song;
# use std::str::FromStr;

let song = Song::from_str(r##"
    {title: Song Title}"
    {artist: The Artist}
    
    This is the first verse.
    You can specify chords using brackets.
    This is a [G]chord

    {soc}
    This is the chorus of the song
    [Em]You can also add some chords
    {eoc}
"##).unwrap();
```

## Supported directives

- Metadata:
    + [X] title (short: t)
    + [ ] subtitle (short: st)
    + [X] artist
    + [ ] composer
    + [ ] lyricist
    + [ ] copyright
    + [ ] album
    + [ ] year
    + [ ] key
    + [ ] time
    + [ ] tempo
    + [ ] duration
    + [X] capo
    + [ ] meta
- Formatting:
    + [X] comment (short: c)
    + [ ] comment_italic (short: ci)
    + [ ] comment_box (short: cb)
    + [ ] image
- Environments:
    + [X] start_of_chorus (short: soc)
    + [X] end_of_chorus (short: eoc)
    + [ ] chorus
    + [ ] start_of_verse
    + [ ] end_of_verse
    + [ ] start_of_tab (short: sot)
    + [ ] end_of_tab (short: eot)
    + [ ] start_of_grid
    + [ ] end_of_grid
- [ ] Custom extensions (prefixed by `x_`)

## Supported chords
At the moment, only A-G notation is supported. Flats (with `b`),
sharps (with `#`) and common postfixes (`m`, `min`, `maj`, numbers).
