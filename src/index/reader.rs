use crate::file_utils::read_line_with_context;
use crate::index;
use crate::index::ix_path;
use crate::index::lines_reader::LinesReader;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Clone)]
pub struct IxToken {
    pub text: String,
    pub(crate) occurrences_offset: usize,
}

pub struct IxReader {
    log_path: PathBuf,
    pub(crate) ix_path: PathBuf,
    pub tokens: Vec<IxToken>,
}

impl IxReader {
    pub fn new(log_path: PathBuf) -> anyhow::Result<Self> {
        let ix_path = ix_path(log_path.clone())?;
        let tokens = Self::read_tokens(ix_path.clone())?;
        Ok(Self {
            log_path,
            ix_path,
            tokens,
        })
    }

    pub fn print_debug(&self) {
        println!("Tokens:");
        for token in &self.tokens {
            println!(
                "  {}: {}-{}",
                token.text, token.occurrences_offset, token.occurrences_offset
            );
        }
    }

    fn read_tokens(ix_path: PathBuf) -> anyhow::Result<Vec<IxToken>> {
        let ix_file = std::fs::File::open(ix_path)?;
        let mut reader = std::io::BufReader::new(ix_file);
        reader.seek(SeekFrom::End(-8))?;
        let tokens_offset = usize::from_be_bytes(index::read_bytes(&mut reader)?);
        reader.seek(SeekFrom::Start(tokens_offset as u64))?;
        let compressed_buf_len = usize::from_be_bytes(index::read_bytes(&mut reader)?);
        let mut compressed_buf = vec![0u8; compressed_buf_len];
        reader.read_exact(&mut compressed_buf)?;
        let mut buf = zstd::decode_all(Cursor::new(&compressed_buf))?;
        let mut tokens_reader = Cursor::new(&mut buf);
        let token_count = usize::from_be_bytes(index::read_bytes(&mut tokens_reader)?);

        let mut tokens = Vec::with_capacity(token_count);
        for _ in 0..token_count {
            let text_len = index::read_bytes::<_, 1>(&mut tokens_reader)?[0] as usize;

            let mut text_bytes = vec![0u8; text_len];
            tokens_reader.read_exact(&mut text_bytes)?;
            let text = String::from_utf8(text_bytes)?;

            let occurrences_offset = usize::from_be_bytes(index::read_bytes(&mut tokens_reader)?);

            tokens.push(IxToken {
                text,
                occurrences_offset,
            });
        }
        Ok(tokens)
    }

    pub fn read_log_line(
        &self,
        line_offset: usize,
        before: usize,
        after: usize,
    ) -> anyhow::Result<(Vec<String>, String, Vec<String>)> {
        read_line_with_context(&self.log_path, line_offset as u64, before, after)
    }

    fn get_tokens(&self, prefix: &str) -> Vec<&IxToken> {
        let mut tokens = Vec::new();
        let mut pos = self
            .tokens
            .binary_search_by_key(&prefix, |x| &x.text)
            .unwrap_or_else(|pos| pos);
        let mut i = pos;
        while i > 0 && self.tokens[i - 1].text.starts_with(&prefix) {
            tokens.push(&self.tokens[i - 1]);
            i -= 1;
        }
        while pos < self.tokens.len() && self.tokens[pos].text.starts_with(&prefix) {
            tokens.push(&self.tokens[pos]);
            pos += 1;
        }
        tokens
    }

    pub fn find_matches(&self, query: &str) -> anyhow::Result<(LinesReader, Vec<String>)> {
        let mut highlight_words = Vec::new();
        let mut alt_lines = Vec::new();
        for words in query.split('|') {
            let mut word_lines = Vec::new();
            for word in words.split_whitespace() {
                highlight_words.push(word.to_string());
                let word_tokens_lines = self
                    .get_tokens(&word.to_uppercase())
                    .into_iter()
                    .map(|x| LinesReader::token(self, x))
                    .collect::<Result<_, _>>()?;
                word_lines.push(LinesReader::any(word_tokens_lines)?);
            }
            alt_lines.push(LinesReader::all(word_lines)?);
        }
        Ok((LinesReader::any(alt_lines)?, highlight_words))
    }
}
