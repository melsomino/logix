mod all;
mod any;
mod word;

use crate::index::reader::IxReader;
use crate::index::words_section::IxWord;
pub use all::AllLinesReader;
pub use any::AnyLinesReader;
pub use word::WordLinesReader;

pub enum LinesReader {
    Word(WordLinesReader),
    Any(AnyLinesReader),
    All(AllLinesReader),
}

impl LinesReader {
    pub fn with_word(ix: &IxReader, word: &IxWord) -> anyhow::Result<Self> {
        Ok(Self::Word(WordLinesReader::new(ix, word)?))
    }

    pub fn with_any(mut readers: Vec<Self>) -> anyhow::Result<Option<Self>> {
        Ok(match readers.len() {
            0 => None,
            1 => readers.pop(),
            _ => Some(Self::Any(AnyLinesReader::new(readers)?)),
        })
    }

    pub fn with_all(mut readers: Vec<Self>) -> anyhow::Result<Option<Self>> {
        Ok(match readers.len() {
            0 => None,
            1 => readers.pop(),
            _ => Some(Self::All(AllLinesReader::new(readers)?)),
        })
    }

    pub fn next(&mut self) -> anyhow::Result<Option<u64>> {
        match self {
            Self::Word(reader) => reader.next(),
            Self::Any(reader) => reader.next(),
            Self::All(reader) => reader.next(),
        }
    }

    pub fn print_debug(&self, indent: usize) {
        match self {
            LinesReader::Word(reader) => reader.print_debug(indent),
            LinesReader::Any(reader) => reader.print_debug(indent),
            LinesReader::All(reader) => reader.print_debug(indent),
        }
    }
}
