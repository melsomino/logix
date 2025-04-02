mod all_tokens;
mod any_token;
mod token;

use crate::index::reader::{IxReader, IxToken};
pub use all_tokens::AllTokensLinesReader;
pub use any_token::AnyTokenLinesReader;
pub use token::TokenLinesReader;

pub enum LinesReader {
    SingleToken(TokenLinesReader),
    AnyToken(AnyTokenLinesReader),
    AllTokens(AllTokensLinesReader),
}

impl LinesReader {
    pub fn token(ix: &IxReader, token: &IxToken) -> anyhow::Result<Self> {
        Ok(Self::SingleToken(TokenLinesReader::new(ix, token)?))
    }

    pub fn any(mut readers: Vec<Self>) -> anyhow::Result<Self> {
        Ok(if readers.len() == 1 {
            readers.pop().unwrap()
        } else {
            Self::AnyToken(AnyTokenLinesReader::new(readers)?)
        })
    }

    pub fn all(mut readers: Vec<Self>) -> anyhow::Result<Self> {
        Ok(if readers.len() == 1 {
            readers.pop().unwrap()
        } else {
            Self::AllTokens(AllTokensLinesReader::new(readers)?)
        })
    }

    pub fn next(&mut self) -> anyhow::Result<Option<usize>> {
        match self {
            Self::SingleToken(reader) => reader.next(),
            Self::AnyToken(reader) => reader.next(),
            Self::AllTokens(reader) => reader.next(),
        }
    }

    pub fn print_debug(&self) {
        self.internal_print_debug(0);
    }

    fn internal_print_debug(&self, indent: usize) {
        let ident_prefix = "  ".repeat(indent);
        match self {
            LinesReader::SingleToken(reader) => {
                println!(
                    "{ident_prefix}{}: {}",
                    reader.token.text, reader.token.occurrences_offset
                );
            }
            LinesReader::AnyToken(readers) => {
                println!("{ident_prefix}Any:");
                for reader in &readers.readers {
                    reader.internal_print_debug(indent + 1);
                }
            }
            LinesReader::AllTokens(readers) => {
                println!("{ident_prefix}All:");
                for reader in &readers.readers {
                    reader.internal_print_debug(indent + 1);
                }
            }
        }
    }
}
