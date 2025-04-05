use crate::index::{ReadEx, WriteEx};
use std::io::{Cursor, Read, Write};

pub struct IxWord {
    pub text: String,
    pub lines_section_offset: u64,
}

impl IxWord {
    pub fn new(text: String, lines_section_offset: u64) -> IxWord {
        Self {
            text,
            lines_section_offset,
        }
    }
}

pub struct IxWordsSection {
    pub words: Vec<IxWord>,
}

impl IxWordsSection {
    pub fn new() -> IxWordsSection {
        Self { words: Vec::new() }
    }

    pub fn write(&self, writer: &mut impl Write) -> anyhow::Result<u64> {
        let mut buf = Vec::new();
        let mut buf_writer = Cursor::new(&mut buf);
        buf_writer.write_u64_be(self.words.len() as u64)?;
        for word in &self.words {
            let text_bytes = word.text.as_bytes();
            buf_writer.write_u8(text_bytes.len() as u8)?;
            buf_writer.write(text_bytes)?;
            buf_writer.write_u64_be(word.lines_section_offset)?;
        }
        Ok(writer.write_compressed(&buf)?)
    }

    pub fn read(reader: &mut impl Read) -> anyhow::Result<Self> {
        let mut buf = reader.read_compressed()?;
        let mut words_reader = Cursor::new(&mut buf);
        let len = words_reader.read_u64_be()? as usize;
        let mut words = Vec::with_capacity(len);
        for _ in 0..len {
            let text_len = words_reader.read_u8()? as usize;
            let mut text_bytes = vec![0u8; text_len];
            words_reader.read_exact(&mut text_bytes)?;
            let text = String::from_utf8(text_bytes)?;
            let lines_section_offset = words_reader.read_u64_be()?;
            words.push(IxWord {
                text,
                lines_section_offset,
            })
        }
        Ok(Self { words })
    }

    pub fn get_prefixed(&self, prefix: &str) -> Vec<&IxWord> {
        let mut tokens = Vec::new();
        let mut pos = self
            .words
            .binary_search_by_key(&prefix, |x| &x.text)
            .unwrap_or_else(|pos| pos);
        let mut i = pos;
        while i > 0 && self.words[i - 1].text.starts_with(&prefix) {
            tokens.push(&self.words[i - 1]);
            i -= 1;
        }
        while pos < self.words.len() && self.words[pos].text.starts_with(&prefix) {
            tokens.push(&self.words[pos]);
            pos += 1;
        }
        tokens
    }
}
