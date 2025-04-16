use crate::file_utils::LogLine;
use colorize::AnsiColor;

pub fn print_line(
    line: LogLine,
    highlight_words: &[String],
    show_separator: &mut bool,
) -> anyhow::Result<()> {
    if !line.before.is_empty() || !line.after.is_empty() {
        if *show_separator {
            println!("--");
        } else {
            *show_separator = true;
        }
    }
    for line in line.before {
        println!("{}", line.b_grey());
    }
    println!(
        "{}",
        crate::print_utils::highlight_words(line.line.as_str(), &highlight_words)
    );
    for line in line.after {
        println!("{}", line.b_grey());
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
