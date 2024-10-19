//! Parser for `Song` related objects
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::str::FromStr;

use crate::song::{Chunk, Line, Paragraph, Section, Song};

use crate::chords::{Chord, Note};

#[derive(Parser)]
#[grammar = "grammar/chordpro.pest"]
pub struct SongParser;

trait HasRule {
    const MATCH_RULE: Rule;
}

trait ProcessChild<'a> {
    //Match and call from_pair
    fn process_child<'m>(&'m mut self, pair: Pair<'a, Rule>);
}

trait FromPair<'a>: HasRule + Sized {
    fn from_pair(content: Pair<'a, Rule>) -> Self;
}

impl<'a, T> FromPair<'a> for T
where
    T: Default + ProcessChild<'a> + HasRule,
{
    fn from_pair(content: Pair<'a, Rule>) -> Self {
        let mut d = Self::default();

        for input in content.into_inner() {
            d.process_child(input);
        }

        d
    }
}

impl FromStr for Song {
    type Err = pest::error::Error<Rule>;
    fn from_str(s: &str) -> Result<Song, Self::Err> {
        let content = SongParser::parse(Song::MATCH_RULE, s)?;

        let pair = content.peek().unwrap();
        Ok(Song::from_pair(pair))
    }
}

impl FromStr for Chord {
    type Err = pest::error::Error<Rule>;
    fn from_str(s: &str) -> Result<Chord, Self::Err> {
        let content = SongParser::parse(Chord::MATCH_RULE, s)?;

        let pair = content.peek().unwrap();
        Ok(Chord::from_pair(pair))
    }
}

impl FromStr for Note {
    type Err = pest::error::Error<Rule>;
    fn from_str(s: &str) -> Result<Note, Self::Err> {
        let content = SongParser::parse(Note::MATCH_RULE, s)?;

        let pair = content.peek().unwrap();
        Ok(Note::from_pair(pair))
    }
}

impl HasRule for Note {
    const MATCH_RULE: Rule = Rule::note;
}

impl<'a> ProcessChild<'a> for Note {
    fn process_child(&mut self, pair: Pair<'a, Rule>) {
        match pair.as_rule() {
            Rule::note_s => {
                let k: Option<char> = pair.as_str().chars().nth(0).map(|c| c.to_ascii_uppercase());

                match k {
                    Some('A') => *self = Note::A,
                    Some('B') => *self = Note::B,
                    Some('C') => *self = Note::C,
                    Some('D') => *self = Note::D,
                    Some('E') => *self = Note::E,
                    Some('F') => *self = Note::F,
                    Some('G') => *self = Note::G,
                    _ => {}
                }
            }

            Rule::sharp => {
                *self = *self + 1;
            }
            Rule::flat => {
                *self = *self - 1;
            }
            _ => {}
        }
    }
}

impl HasRule for Chord {
    const MATCH_RULE: Rule = Rule::chord;
}

impl<'a> ProcessChild<'a> for Chord {
    fn process_child(&mut self, pair: Pair<'a, Rule>) {
        match pair.as_rule() {
            Rule::note => {
                let note = Note::from_pair(pair);
                self.root = note;
                self.bass = note;
            }
            Rule::major => {
                self.minor = false;
            }
            Rule::minor => {
                self.minor = true;
            }
            Rule::symbol => {
                self.others = pair.as_str().to_owned();
            }
            Rule::number => {
                self.number = u8::from_str(pair.as_str()).unwrap();
            }
            Rule::bass => {
                self.bass = Note::from_pair(pair.into_inner().peek().unwrap());
            }
            _ => {}
        }
    }
}

impl HasRule for Line {
    const MATCH_RULE: Rule = Rule::line;
}

impl<'a> ProcessChild<'a> for Line {
    fn process_child(&mut self, pair: Pair<'a, Rule>) {
        match pair.as_rule() {
            Rule::chord => {
                self.0.push(Chunk::Chord(Chord::from_pair(pair)));
            }
            Rule::text => {
                self.0.push(Chunk::Lyrics(pair.as_str().to_owned()));
            }
            _ => {}
        }
    }
}

impl HasRule for Paragraph {
    const MATCH_RULE: Rule = Rule::paragraph;
}

impl<'a> ProcessChild<'a> for Paragraph {
    fn process_child(&mut self, pair: Pair<'a, Rule>) {
        if let Rule::line = pair.as_rule() {
            self.0.push(Line::from_pair(pair));
        }
    }
}

impl HasRule for Section {
    const MATCH_RULE: Rule = Rule::section;
}

impl<'a> FromPair<'a> for Section {
    fn from_pair(content: Pair<'a, Rule>) -> Self {
        let pair = content.into_inner().peek().unwrap();
        let rule = pair.as_rule();

        match rule {
            Rule::paragraph => Section::Verse(Paragraph::from_pair(pair)),
            Rule::chorus => {
                Section::Chorus(Paragraph::from_pair(pair.into_inner().peek().unwrap()))
            }
            Rule::comment => Section::Comment(Line::from_pair(pair.into_inner().peek().unwrap())),
            _ => Section::Comment(Line::default()),
        }
    }
}

impl Song {
    fn parse_directive<'a, 'b>(&'b mut self, mut pairs: Pairs<'a, Rule>) {
        let name = pairs.next();
        let data = pairs.next();

        if let Some(name) = name {
            if name.as_rule() == Rule::directive_name {
                match name.as_str() {
                    "title" => {
                        self.title = data.map(|x| x.as_str()).unwrap_or("").to_owned();
                    },
                    "artist" => {
                        self.artist = data.map(|x| x.as_str()).unwrap_or("").to_owned();
                    },
                    "capo" => {
                        let capo_str = data.map(|x| x.as_str()).unwrap_or("");

                        if let Ok(capo) = u8::from_str(capo_str) {
                            self.capo = capo;
                        } else {
                            self.capo = 0;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

impl HasRule for Song {
    const MATCH_RULE: Rule = Rule::chordpro;
}

impl<'a> ProcessChild<'a> for Song {
    fn process_child(&mut self, pair: Pair<'a, Rule>) {
        match pair.as_rule() {
            Rule::directive => {
                self.parse_directive(pair.into_inner());
            }
            Rule::section => {
                self.song.push(Section::from_pair(pair));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! chord {
        ($s:expr) => {
            Chord::from_str($s).expect("Failed to create chord")
        }
    }

    macro_rules! parse_test {
        ($type:ty { 
            $line:expr => $result:expr
        }) => {{
            let parsed: $type = SongParser::parse_str($line).expect("Failed to parse");
            assert_eq!(parsed, $result);
        }};
    }
    
    impl SongParser {
        fn parse_str<'a, T>(s: &'a str) -> Result<T, pest::error::Error<Rule>>
        where
            T: HasRule + FromPair<'a>,
        {
            let content = SongParser::parse(T::MATCH_RULE, s)?;

            let pair = content.peek().unwrap();
            Ok(T::from_pair(pair))
        }
    }
    #[test]
    fn test_line_parse() {
        parse_test!(Line {
            "[C]How I wish, how I wish you were [D]here"
            =>
            Line(vec![
                Chunk::Chord(chord!("C")),
                Chunk::Lyrics("How I wish, how I wish you were ".to_string()),
                Chunk::Chord(chord!("D")),
                Chunk::Lyrics("here".to_string())
            ])
        })
    }

    #[test]
    fn test_paragraph_parse() {
        parse_test!( Paragraph {
            r#"[C]How I wish, how I wish you were [D]here
            We're just [Am]two lost souls swimming in a fish bowl,[G] year after year"#
            =>
            Paragraph(vec![
                Line(vec![
                    Chunk::Chord(chord!("C")),
                    Chunk::Lyrics("How I wish, how I wish you were ".to_string()),
                    Chunk::Chord(chord!("D")),
                    Chunk::Lyrics("here".to_string())
                ]),
                Line(vec![
                    Chunk::Lyrics("We're just ".to_string()),
                    Chunk::Chord(chord!("Am")),
                    Chunk::Lyrics("two lost souls swimming in a fish bowl,".to_string()),
                    Chunk::Chord(chord!("G")),
                    Chunk::Lyrics(" year after year".to_string())
                ])
            ])
        })
    }

    #[test]
    fn test_chorus_parse() {
        parse_test!( Section {
            r#"{soc}
            [C]How I wish, how I wish you were [D]here
            We're just [Am]two lost souls swimming in a fish bowl,[G] year after year
            {eoc}"#
            =>
            Section::Chorus(Paragraph(vec![
                Line(vec![
                    Chunk::Chord(chord!("C")),
                    Chunk::Lyrics("How I wish, how I wish you were ".to_string()),
                    Chunk::Chord(chord!("D")),
                    Chunk::Lyrics("here".to_string())
                ]),
                Line(vec![
                    Chunk::Lyrics("We're just ".to_string()),
                    Chunk::Chord(chord!("Am")),
                    Chunk::Lyrics("two lost souls swimming in a fish bowl,".to_string()),
                    Chunk::Chord(chord!("G")),
                    Chunk::Lyrics(" year after year".to_string())
                ])
            ]))
        })
    }

    #[test]
    fn test_comment_parse() {
        parse_test!( Section {
            r#"{comment: This is a comment [Am]}"#
            =>
            Section::Comment(
                Line(vec![
                    Chunk::Lyrics("This is a comment ".to_string()),
                    Chunk::Chord(chord!("Am")),
                ]),
            )
        })
    }

    #[test]
    fn test_song_parse() {
        parse_test!( Song {
            r#"{title: Wish You Were Here}
            {artist: Pink Floyd}

            {soc}
            [C]How I wish, how I wish you were [D]here
            We're just [Am]two lost souls swimming in a fish bowl,[G] year after year
            {eoc}"#
            => Song{
                title: "Wish You Were Here".to_string(),
                artist: "Pink Floyd".to_string(),
                capo: 0,
                song: vec![Section::Chorus(Paragraph(vec![
                    Line(vec![
                        Chunk::Chord(chord!("C")),
                        Chunk::Lyrics("How I wish, how I wish you were ".to_string()),
                        Chunk::Chord(chord!("D")),
                        Chunk::Lyrics("here".to_string())
                    ]),
                    Line(vec![
                        Chunk::Lyrics("We're just ".to_string()),
                        Chunk::Chord(chord!("Am")),
                        Chunk::Lyrics("two lost souls swimming in a fish bowl,".to_string()),
                        Chunk::Chord(chord!("G")),
                        Chunk::Lyrics(" year after year".to_string())
                    ])
                ]))]
            }
        })
    }

    #[test]
    fn test_unknown_directive() {
        parse_test!( Song {
            r#"{directive not known}
            {other: directive not known}
            [C]How I wish, how I wish you were [D]here
            We're just [Am]two lost souls swimming in a fish bowl,[G] year after year"#
            => Song{
                title: "".to_string(),
                artist: "".to_string(),
                capo: 0,
                song: vec![Section::Verse(Paragraph(vec![
                    Line(vec![
                        Chunk::Chord(chord!("C")),
                        Chunk::Lyrics("How I wish, how I wish you were ".to_string()),
                        Chunk::Chord(chord!("D")),
                        Chunk::Lyrics("here".to_string())
                    ]),
                    Line(vec![
                        Chunk::Lyrics("We're just ".to_string()),
                        Chunk::Chord(chord!("Am")),
                        Chunk::Lyrics("two lost souls swimming in a fish bowl,".to_string()),
                        Chunk::Chord(chord!("G")),
                        Chunk::Lyrics(" year after year".to_string())
                    ])
                ]))]
            }
        })
    }

    #[test]
    fn test_chord() {
        parse_test!( Chord {
            "C#dim4/G"
            => Chord{
                root: Note::CSharp,
                bass: Note::G,
                minor: false,
                number: 4,
                others: "dim".to_string()
            }
        })
    }
}
