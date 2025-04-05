use crate::file_utils::read_line_with_context;
use crate::index::header_section::IxHeaderSection;
use crate::index::ix_path;
use crate::index::lines_reader::LinesReader;
use crate::index::words_section::IxWordsSection;
use crate::query::Query;
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;

pub struct IxReader {
    log_path: PathBuf,
    pub ix_path: PathBuf,
    pub words_section: IxWordsSection,
}

impl IxReader {
    pub fn new(log_path: PathBuf) -> anyhow::Result<Self> {
        let ix_path = ix_path(log_path.clone())?;
        let mut file = std::fs::File::open(ix_path.clone())?;
        let header = IxHeaderSection::read(&mut file)?;
        file.seek(SeekFrom::Start(header.words_section_offset as u64))?;
        let words_section = IxWordsSection::read(&mut file)?;
        Ok(Self {
            log_path,
            ix_path,
            words_section,
        })
    }

    pub fn print_debug(&self) {
        println!("Tokens:");
        for word in &self.words_section.words {
            println!("  {}", word.text);
        }
    }

    pub fn read_log_line(
        &self,
        line_offset: u64,
        before: usize,
        after: usize,
    ) -> anyhow::Result<(Vec<String>, String, Vec<String>)> {
        read_line_with_context(&self.log_path, line_offset, before, after)
    }

    pub fn query(&self, query: &Query) -> anyhow::Result<Option<LinesReader>> {
        match query {
            Query::Word(word) => LinesReader::with_any(
                self.words_section
                    .get_prefixed(&word)
                    .into_iter()
                    .map(|x| LinesReader::with_word(self, x))
                    .collect::<Result<_, _>>()?,
            ),
            Query::Any(queries) => LinesReader::with_any(
                queries
                    .into_iter()
                    .filter_map(|x| self.query(x).transpose())
                    .collect::<Result<_, _>>()?,
            ),
            Query::All(queries) => LinesReader::with_all(
                queries
                    .into_iter()
                    .filter_map(|x| self.query(x).transpose())
                    .collect::<Result<_, _>>()?,
            ),
        }
    }
}
