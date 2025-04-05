use crate::index::header_section::IxHeaderSection;
use crate::index::lines_section::IxLinesSection;
use crate::index::words_section::{IxWord, IxWordsSection};
use std::io::{Seek, Write};

pub struct IxBuilder {
    pub words_section: IxWordsSection,
    pub lines_sections: Vec<IxLinesSection>,
}

impl IxBuilder {
    pub fn new() -> Self {
        Self {
            words_section: IxWordsSection::new(),
            lines_sections: Vec::new(),
        }
    }

    pub(crate) fn add_word(&mut self, text: String, line_offset: u64) {
        match self
            .words_section
            .words
            .binary_search_by_key(&&text, |x| &x.text)
        {
            Ok(index) => {
                self.lines_sections[index].add_line_offset(line_offset);
            }
            Err(index) => {
                self.words_section.words.insert(index, IxWord::new(text, 0));
                let mut lines_section = IxLinesSection::new();
                lines_section.add_line_offset(line_offset);
                self.lines_sections.insert(index, IxLinesSection::new());
            }
        }
    }

    pub fn write<W: Write + Seek>(&mut self, writer: &mut W) -> anyhow::Result<()> {
        let start_position = writer.stream_position()?;
        let mut header_section = IxHeaderSection::new();
        let mut pos = start_position + header_section.write(writer)?;
        let words_mut = self.words_section.words.iter_mut();
        for (word, lines_section) in words_mut.zip(self.lines_sections.iter_mut()) {
            word.lines_section_offset = pos;
            pos += lines_section.write(writer)?;
        }
        header_section.words_section_offset = pos;
        self.words_section.write(writer)?;
        let end_position = writer.stream_position()?;
        writer.seek(std::io::SeekFrom::Start(start_position))?;
        header_section.write(writer)?;
        writer.seek(std::io::SeekFrom::Start(end_position))?;
        Ok(())
    }
}
