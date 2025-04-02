use crate::index::{ix_path, IxBuilder};
use crate::token::tokenize;
use std::io::{BufWriter, Write};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn check_index(log_path: PathBuf) -> anyhow::Result<()> {
    let ix_path = ix_path(log_path.clone())?;
    if ix_path.exists() {
        return Ok(());
    }

    let log_file = File::open(&log_path)?;
    let log_size = log_file.metadata()?.len() as usize;
    let mut log_reader = BufReader::new(log_file);
    let mut ix_builder = IxBuilder::new();
    let mut line_offset = 0usize;
    let mut last_percent = 0;
    loop {
        let percent = line_offset * 100 / log_size;
        if percent != last_percent {
            print!("\rIndexing: {}%\x1b[K", percent);
            std::io::stdout().flush()?;
            last_percent = percent;
        }
        let mut line: String = String::new();
        let len = log_reader.read_line(&mut line)?;
        if len == 0 {
            break;
        };
        for token in tokenize(&line) {
            ix_builder.add_token(token, line_offset);
        }
        line_offset += len;
    }
    println!();
    print!("Writing index...");
    let ix_file = File::create(ix_path)?;
    ix_builder.write(&mut BufWriter::new(ix_file))?;
    println!();
    Ok(())
}
