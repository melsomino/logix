use crate::build;
use crate::index::IxReader;
use crate::path_utils::resolve_log_files;
use colorize::AnsiColor;
use std::path::PathBuf;

pub fn query(
    path: PathBuf,
    query: &str,
    print_debug: bool,
    before: usize,
    after: usize,
) -> anyhow::Result<()> {
    let log_paths = resolve_log_files(vec![path])?;
    let print_header = log_paths.len() > 1;
    let mut is_first = true;
    for log_path in log_paths {
        if print_header {
            if !is_first {
                println!();
            } else {
                is_first = false;
            }
            println!("{}:", log_path.display());
            println!();
        }
        build::check_index(log_path.clone())?;
        find_matches(log_path, query, print_debug, before, after)?;
    }
    Ok(())
}

fn find_matches(
    log_path: PathBuf,
    query: &str,
    print_debug: bool,
    before: usize,
    after: usize,
) -> anyhow::Result<()> {
    let ix = IxReader::new(log_path)?;
    if print_debug {
        ix.print_debug();
    }
    let (mut lines, words) = ix.find_matches(query)?;
    if print_debug {
        lines.print_debug();
    }
    let mut is_first = true;
    while let Some(line_offset) = lines.next()? {
        let (before, line, after) = ix.read_log_line(line_offset, before, after)?;
        if !before.is_empty() || !after.is_empty() {
            if is_first {
                is_first = false;
            } else {
                println!("--");
            }
        }
        for line in before {
            println!("{}", line);
        }
        println!("{}", highlight_words(line.as_str(), &words));
        for line in after {
            println!("{}", line);
        }
    }
    Ok(())
}

fn highlight_words(line: &str, words: &[String]) -> String {
    let lowercase_line = line.to_lowercase();
    let lowercase_words: Vec<String> = words.iter().map(|w| w.to_lowercase()).collect();

    let mut result = String::new();
    let mut i = 0;

    while i < line.len() {
        let mut matched = None;

        for (j, word) in lowercase_words.iter().enumerate() {
            if lowercase_line[i..].starts_with(word) {
                matched = Some((word.len(), words[j].clone()));
                break;
            }
        }

        if let Some((len, _)) = matched {
            let part = line[i..i + len].to_string().red();
            result += part.as_str();
            i += len;
        } else {
            let c = line[i..].chars().next().unwrap();
            result.push(c);
            i += c.len_utf8();
        }
    }

    result
}
