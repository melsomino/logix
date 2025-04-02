use serde::{Deserialize, Serialize};
use std::io::{Cursor, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct IxTokenBuilder {
    pub text: String,
    pub occurrences: Vec<usize>,
}

pub struct IxBuilder {
    pub tokens: Vec<IxTokenBuilder>,
}

impl IxBuilder {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }
    pub(crate) fn add_token(&mut self, text: String, line_offset: usize) {
        match self.tokens.binary_search_by_key(&&text, |x| &x.text) {
            Ok(index) => {
                self.tokens[index].occurrences.push(line_offset);
            }
            Err(index) => self.tokens.insert(
                index,
                IxTokenBuilder {
                    text,
                    occurrences: vec![line_offset],
                },
            ),
        }
    }

    pub(crate) fn write<W: Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        let mut occurrences_section_offsets = Vec::new();
        let mut pos = 0;
        for token in &self.tokens {
            let mut buf = vec![0u8; token.occurrences.len() * 5];
            let mut offset = 0;
            for line_offset in &token.occurrences {
                buf[offset..offset + 5].copy_from_slice(&line_offset.to_le_bytes()[0..5]);
                offset += 5;
            }
            let compressed_buf = zstd::encode_all(buf.as_slice(), zstd::DEFAULT_COMPRESSION_LEVEL)?;
            writer.write(&compressed_buf.len().to_be_bytes().as_ref())?;
            writer.write(&compressed_buf)?;
            writer.write(&0usize.to_be_bytes().as_ref())?;
            occurrences_section_offsets.push(pos);
            pos += 8 + compressed_buf.len() + 8;
        }

        let mut buf = Vec::new();
        let mut buf_writer = Cursor::new(&mut buf);
        buf_writer.write(self.tokens.len().to_be_bytes().as_ref())?;
        for (i, token) in self.tokens.iter().enumerate() {
            let text_bytes = token.text.as_bytes();
            buf_writer.write(&[text_bytes.len() as u8])?;
            buf_writer.write(text_bytes)?;
            buf_writer.write(&occurrences_section_offsets[i].to_be_bytes())?;
        }
        let compressed_buf = zstd::encode_all(buf.as_slice(), zstd::DEFAULT_COMPRESSION_LEVEL)?;
        writer.write(&compressed_buf.len().to_be_bytes().as_ref())?;
        writer.write(&compressed_buf)?;
        writer.write(&pos.to_be_bytes().as_ref())?;

        Ok(())
    }
}
