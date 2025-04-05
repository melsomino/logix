mod builder;
mod header_section;
mod lines_reader;
mod lines_section;
mod reader;
mod words_section;

pub use builder::IxBuilder;
pub use reader::IxReader;
use std::io::{Cursor, Read, Write};
use std::path::PathBuf;

// Ix file structure:
// Header:
//   version: u16
//   words_section_offset: u64
// Words section:
//
// Lines section:
//

pub trait ReadEx {
    fn read_u8(&mut self) -> anyhow::Result<u8>;
    fn read_u64_be(&mut self) -> anyhow::Result<u64>;
    fn read_u16_be(&mut self) -> anyhow::Result<u16>;
    fn read_compressed(&mut self) -> anyhow::Result<Vec<u8>>;
}

impl<R: Read> ReadEx for R {
    fn read_u8(&mut self) -> anyhow::Result<u8> {
        let mut buf = [0u8; 1];
        self.read(&mut buf)?;
        Ok(buf[0])
    }

    fn read_u64_be(&mut self) -> anyhow::Result<u64> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    fn read_u16_be(&mut self) -> anyhow::Result<u16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }
    fn read_compressed(&mut self) -> anyhow::Result<Vec<u8>> {
        let mut compressed = vec![0u8; self.read_u64_be()? as usize];
        self.read_exact(&mut compressed)?;
        Ok(zstd::decode_all(Cursor::new(&compressed))?)
    }
}

pub trait WriteEx {
    fn write_u8(&mut self, v: u8) -> anyhow::Result<()>;
    fn write_u16_be(&mut self, v: u16) -> anyhow::Result<()>;
    fn write_u64_be(&mut self, v: u64) -> anyhow::Result<()>;
    fn write_compressed(&mut self, data: &[u8]) -> anyhow::Result<u64>;
}

impl<W: Write> WriteEx for W {
    fn write_u8(&mut self, v: u8) -> anyhow::Result<()> {
        self.write(&[v])?;
        Ok(())
    }

    fn write_u16_be(&mut self, v: u16) -> anyhow::Result<()> {
        self.write(&v.to_be_bytes())?;
        Ok(())
    }

    fn write_u64_be(&mut self, v: u64) -> anyhow::Result<()> {
        self.write(&v.to_be_bytes())?;
        Ok(())
    }

    fn write_compressed(&mut self, data: &[u8]) -> anyhow::Result<u64> {
        let compressed = zstd::encode_all(data, zstd::DEFAULT_COMPRESSION_LEVEL)?;
        self.write_u64_be(compressed.len() as u64)?;
        self.write(&compressed)?;
        Ok(8 + compressed.len() as u64)
    }
}

pub fn ix_path(mut path: PathBuf) -> anyhow::Result<PathBuf> {
    path.set_file_name(format!(
        "{}.ix",
        path.file_name()
            .ok_or_else(|| anyhow::anyhow!("Missing file name"))?
            .to_string_lossy()
    ));
    Ok(path)
}
