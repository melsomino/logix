use crate::index;
use crate::index::reader::{IxReader, IxToken};
use std::io::{Cursor, Read, Seek};

pub struct TokenLinesReader {
    pub(crate) token: IxToken,
    buf: Vec<u8>,
    offset: usize,
    _next_buf_offset: usize,
}

impl TokenLinesReader {
    pub(crate) fn new(ix: &IxReader, token: &IxToken) -> anyhow::Result<Self> {
        let file = std::fs::File::open(ix.ix_path.clone())?;
        let mut reader = std::io::BufReader::new(file);
        reader.seek(std::io::SeekFrom::Start(token.occurrences_offset as u64))?;
        let compressed_len = usize::from_be_bytes(index::read_bytes(&mut reader)?);
        let mut compressed_buf = vec![0u8; compressed_len];
        reader.read_exact(&mut compressed_buf)?;
        let buf = zstd::decode_all(Cursor::new(&compressed_buf))?;
        let next_buf_offset = usize::from_be_bytes(index::read_bytes(&mut reader)?);

        Ok(Self {
            token: token.clone(),
            buf,
            offset: 0,
            _next_buf_offset: next_buf_offset,
        })
    }

    pub(crate) fn next(&mut self) -> anyhow::Result<Option<usize>> {
        if self.offset >= self.buf.len() {
            return Ok(None);
        }
        let mut line_offset_buf = [0u8; 8];
        line_offset_buf[0..5].copy_from_slice(&self.buf[self.offset..self.offset + 5]);
        let line_offset = usize::from_le_bytes(line_offset_buf);
        self.offset += 5;
        Ok(Some(line_offset))
    }
}
