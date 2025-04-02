pub fn tokenize(line: &str) -> Vec<String> {
    // Tokenizer rules:
    // - skip spaces and punctuations
    // - read while alphanumeric
    // - skip token if it contains only digits
    // - skip token if it contains only hex characters and is longer than 8 chars
    // - skip token if len > 4 and more than half of chars are digits
    // - skip ANSI escape/control sequences

    let line = strip_ansi_escape_sequences(line);

    let mut tokens = Vec::new();
    let mut start = None;
    let mut chars = line.char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        if c.is_alphanumeric() {
            if start.is_none() {
                start = Some(i);
            }
        } else if let Some(start_i) = start.take() {
            let word = &line[start_i..i];
            if should_skip(word) {
                continue;
            }
            tokens.push(word.to_ascii_uppercase());
        }
    }

    if let Some(start_i) = start {
        let word = &line[start_i..];
        if !should_skip(word) {
            tokens.push(word.to_ascii_uppercase());
        }
    }

    tokens
}

fn should_skip(word: &str) -> bool {
    if word.chars().all(|c| c.is_ascii_digit()) {
        return true;
    }
    if word.len() > 8 && word.chars().all(|c| c.is_ascii_hexdigit()) {
        return true;
    }
    if word.len() > 4 {
        let digit_count = word.chars().filter(|c| c.is_ascii_digit()).count();
        if digit_count * 2 > word.len() {
            return true;
        }
    }
    false
}

fn strip_ansi_escape_sequences(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' || c == '\u{1b}' {
            if chars.peek() == Some(&'[') {
                // Consume CSI sequence
                chars.next();
                while let Some(&next) = chars.peek() {
                    if ('a'..='z').contains(&next) || ('A'..='Z').contains(&next) {
                        chars.next(); // consume the final char
                        break;
                    }
                    chars.next();
                }
                continue;
            }
        }
        out.push(c);
    }

    out
}
