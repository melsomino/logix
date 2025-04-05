use crate::index::{ReadEx, WriteEx};
use std::io::{Read, Write};

pub struct IxLinesSection {
    pub next_section_offset: usize,
    pub line_offsets_buf: Vec<u8>,
}

impl IxLinesSection {
    pub fn new() -> Self {
        Self {
            next_section_offset: 0,
            line_offsets_buf: Vec::new(),
        }
    }

    pub fn write(&self, writer: &mut impl Write) -> anyhow::Result<usize> {
        writer.write_u64_be(self.next_section_offset)?;
        Ok(8 + writer.write_compressed(&self.line_offsets_buf)?)
    }

    pub fn read(reader: &mut impl Read) -> anyhow::Result<Self> {
        let next_section_offset = reader.read_u64_be()?;
        let line_offsets_buf = reader.read_compressed()?;
        Ok(Self {
            next_section_offset,
            line_offsets_buf,
        })
    }

    pub fn add_line_offset(&mut self, offset: usize) {
        let mut offset_buf = [0u8; 5];
        offset_buf.copy_from_slice(&(offset as u64).to_le_bytes()[0..5]);
        self.line_offsets_buf.extend(&offset_buf);
    }

    pub fn get_line_offset(&self, buf_offset: usize) -> u64 {
        let mut buf = [0u8; 8];
        buf[0..5].copy_from_slice(&self.line_offsets_buf[buf_offset..buf_offset + 5]);
        u64::from_le_bytes(buf)
    }
}
