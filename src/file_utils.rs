use crate::query::{Query, parse_words};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

pub struct LogLine {
    pub line: String,
    pub before: Vec<String>,
    pub after: Vec<String>,
}

impl LogLine {
    pub fn read(
        path: impl AsRef<Path>,
        offset: u64,
        before: usize,
        after: usize,
        check_order: Option<&Query>,
    ) -> anyhow::Result<Option<Self>> {
        let file = std::fs::File::open(path)?;
        let mut reader = BufReader::new(&file);

        reader.seek(SeekFrom::Start(offset))?;
        let mut target_line = String::new();
        reader.read_line(&mut target_line)?;
        if let Some(query) = check_order {
            if !query.check_words_order(&parse_words(&target_line)) {
                return Ok(None);
            }
        }

        let lines_before = read_lines_before(&file, offset, before)?;

        let mut lines_after = Vec::new();
        for _ in 0..after {
            let mut line = String::new();
            if reader.read_line(&mut line)? == 0 {
                break;
            }
            lines_after.push(line.trim_end().to_string());
        }

        Ok(Some(Self {
            before: lines_before,
            line: target_line.trim_end().to_string(),
            after: lines_after,
        }))
    }
}

fn read_lines_before(
    file: &std::fs::File,
    offset: u64,
    before: usize,
) -> std::io::Result<Vec<String>> {
    if before == 0 {
        return Ok(Vec::new());
    }
    let mut result = Vec::new();
    let mut cursor = file.try_clone()?;
    let mut pos = offset;
    let mut buf = Vec::new();

    while pos > 0 && result.len() < before {
        let step = pos.min(1024);
        pos -= step;
        cursor.seek(SeekFrom::Start(pos))?;

        buf.resize(step as usize, 0);
        cursor.read_exact(&mut buf)?;

        for (i, b) in buf.iter().enumerate().rev() {
            if *b == b'\n' {
                let line_start = pos + i as u64 + 1;
                if line_start < offset {
                    let mut r = BufReader::new(file.try_clone()?);
                    r.seek(SeekFrom::Start(line_start))?;
                    let mut line = String::new();
                    r.read_line(&mut line)?;
                    result.push(line.trim_end().to_string());
                    if result.len() == before {
                        break;
                    }
                }
            }
        }
    }

    result.reverse();
    Ok(result)
}
