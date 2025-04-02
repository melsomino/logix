use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

pub fn read_line_with_context(
    path: impl AsRef<Path>,
    offset: u64,
    before: usize,
    after: usize,
) -> anyhow::Result<(Vec<String>, String, Vec<String>)> {
    let file = std::fs::File::open(path)?;
    let mut reader = BufReader::new(&file);

    // 1. Seek to the offset
    reader.seek(SeekFrom::Start(offset))?;

    // 2. Read the target line
    let mut target_line = String::new();
    reader.read_line(&mut target_line)?;

    // 3. Print N lines before
    let lines_before = if before > 0 {
        read_lines_before(&file, offset, before)?
    } else {
        Vec::new()
    };
    read_lines_before(&file, offset, before)?;

    let mut lines_after = Vec::new();
    // 5. Print M lines after
    for _ in 0..after {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        lines_after.push(line.trim_end().to_string());
    }

    Ok((
        lines_before,
        target_line.trim_end().to_string(),
        lines_after,
    ))
}

fn read_lines_before(
    file: &std::fs::File,
    offset: u64,
    before: usize,
) -> std::io::Result<Vec<String>> {
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
