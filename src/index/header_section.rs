use crate::index::{ReadEx, WriteEx};
use std::io::{Read, Write};

pub struct IxHeaderSection {
    pub version: u16,
    pub words_section_offset: u64,
}

impl IxHeaderSection {
    pub fn new() -> Self {
        Self {
            version: 0,
            words_section_offset: 0,
        }
    }

    pub fn read(reader: &mut impl Read) -> anyhow::Result<Self> {
        let version = reader.read_u16_be()?;
        let words_section_offset = reader.read_u64_be()?;
        Ok(Self {
            version,
            words_section_offset,
        })
    }

    pub fn write(&self, writer: &mut impl Write) -> anyhow::Result<u64> {
        writer.write_u16_be(self.version)?;
        writer.write_u64_be(self.words_section_offset)?;
        Ok(2 + 8)
    }
}
