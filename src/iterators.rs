use crate::song::{
    Song,
    Section,
    Line,
    Chunk,
};
use std::slice::Iter;
use std::slice::IterMut;
use std::iter::{Once, once};

impl Song {
    pub fn iter(&self) -> Iter<Section> {
        self.song.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Section> {
        self.song.iter_mut()
    }
}

pub enum SectionIterator<'a> {
    Paragraph(Iter<'a, Line>),
    Line(Once<&'a Line>)
}

pub enum SectionMutIterator<'a> {
    Paragraph(IterMut<'a, Line>),
    Line(Once<&'a mut Line>)
}

impl Section {
    pub fn iter(&self) -> SectionIterator {
        match self {
            Section::Chorus(p) => SectionIterator::Paragraph(p.0.iter()),
            Section::Verse(p) => SectionIterator::Paragraph(p.0.iter()),
            Section::Comment(l) => SectionIterator::Line(once(l))
        }
    }

    pub fn iter_mut(&mut self) -> SectionMutIterator {
        match self {
            Section::Chorus(p) => SectionMutIterator::Paragraph(p.0.iter_mut()),
            Section::Verse(p) => SectionMutIterator::Paragraph(p.0.iter_mut()),
            Section::Comment(l) => SectionMutIterator::Line(once(l))
        }
    }
}

impl <'a> Iterator for SectionIterator<'a> {
    type Item = &'a Line;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SectionIterator::Paragraph(p) => p.next(),
            SectionIterator::Line(l) => l.next()
        }
    }
}

impl <'a> Iterator for SectionMutIterator<'a> {
    type Item = &'a mut Line;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SectionMutIterator::Paragraph(p) => p.next(),
            SectionMutIterator::Line(l) => l.next()
        }
    }
}

impl Line {
    pub fn iter(&self) -> Iter<Chunk> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Chunk> {
        self.0.iter_mut()
    }
}