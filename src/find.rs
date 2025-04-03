use crate::build;
use crate::index::IxReader;
use crate::path_utils::resolve_log_files;
use colorize::AnsiColor;
use std::collections::VecDeque;
use std::path::PathBuf;

pub fn query(
    path: PathBuf,
    query: &str,
    print_debug: bool,
    before: usize,
    after: usize,
    head: usize,
    tail: usize,
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
        query_file(log_path, query, print_debug, before, after, head, tail)?;
    }
    Ok(())
}

fn query_file(
    log_path: PathBuf,
    query: &str,
    print_debug: bool,
    before: usize,
    after: usize,
    head: usize,
    tail: usize,
) -> anyhow::Result<()> {
    let ix = IxReader::new(log_path)?;
    if print_debug {
        ix.print_debug();
    }
    let (mut lines, words) = ix.find_matches(query)?;
    let mut tail_lines = VecDeque::new();
    if print_debug {
        lines.print_debug();
    }
    let mut show_separator = true;
    let mut processed = 0;
    let head_requested = head > 0;
    let tail_requested = tail > 0;
    while let Some(line_offset) = lines.next()? {
        if head_requested && processed < head || !tail_requested {
            print_line(&ix, line_offset, &words, before, after, &mut show_separator)?;
        }
        if tail_requested && (!head_requested || processed >= head) {
            tail_lines.push_back(line_offset);
            if tail_lines.len() > tail {
                tail_lines.pop_front();
            }
        }
        processed += 1;
        if !tail_requested && head_requested && processed >= head {
            break;
        }
    }
    for line_offset in tail_lines {
        print_line(&ix, line_offset, &words, before, after, &mut show_separator)?;
    }
    Ok(())
}

fn print_line(
    ix: &IxReader,
    line_offset: usize,
    highlight_words: &[String],
    before: usize,
    after: usize,
    show_separator: &mut bool,
) -> anyhow::Result<()> {
    let (before, line, after) = ix.read_log_line(line_offset, before, after)?;
    if !before.is_empty() || !after.is_empty() {
        if *show_separator {
            println!("--");
            *show_separator = false;
        }
    }
    for line in before {
        println!("{}", line);
    }
    println!(
        "{}",
        crate::find::highlight_words(line.as_str(), &highlight_words)
    );
    for line in after {
        println!("{}", line);
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
