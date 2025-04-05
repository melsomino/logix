use crate::index::IxReader;
use colorize::AnsiColor;

pub fn print_line(
    ix: &IxReader,
    line_offset: u64,
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
        crate::print_utils::highlight_words(line.as_str(), &highlight_words)
    );
    for line in after {
        println!("{}", line);
    }
    Ok(())
}

pub fn highlight_words(line: &str, words: &[String]) -> String {
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
