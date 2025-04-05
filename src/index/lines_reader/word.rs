use crate::index::lines_section::IxLinesSection;
use crate::index::reader::IxReader;
use crate::index::words_section::IxWord;
use std::io::Seek;

pub struct WordLinesReader {
    word: String,
    lines_section: IxLinesSection,
    buf_offset: usize,
}

impl WordLinesReader {
    pub fn new(ix: &IxReader, word: &IxWord) -> anyhow::Result<Self> {
        let mut file = std::fs::File::open(ix.ix_path.clone())?;
        file.seek(std::io::SeekFrom::Start(word.lines_section_offset as u64))?;
        Ok(Self {
            word: word.text.clone(),
            lines_section: IxLinesSection::read(&mut file)?,
            buf_offset: 0,
        })
    }

    pub fn next(&mut self) -> anyhow::Result<Option<u64>> {
        if self.buf_offset >= self.lines_section.line_offsets_buf.len() {
            return Ok(None);
        }
        let offset = self.lines_section.get_line_offset(self.buf_offset);
        self.buf_offset += 5;
        Ok(Some(offset))
    }

    pub(crate) fn print_debug(&self, indent: usize) {
        println!("{}{}", "  ".repeat(indent), self.word)
    }
}
