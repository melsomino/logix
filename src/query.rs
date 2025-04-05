pub enum Query {
    Word(String),
    Any(Vec<Query>),
    All(Vec<Query>),
}

fn reduce(queries: Vec<Query>, combine: impl FnOnce(Vec<Query>) -> Query) -> Option<Query> {
    match queries.len() {
        0 => None,
        1 => queries.into_iter().next(),
        _ => Some(combine(queries)),
    }
}

impl Query {
    pub fn parse(query_str: &str) -> Option<Self> {
        let mut any = Vec::new();
        for any_str in query_str.split('|') {
            let mut words = Vec::new();
            for word in parse_words(any_str) {
                words.push(Query::with_word(word));
            }
            if let Some(all) = Query::with_all(words) {
                any.push(all);
            }
        }
        Self::with_any(any)
    }

    pub fn with_word(word: String) -> Self {
        Self::Word(word)
    }

    pub fn with_any(queries: Vec<Query>) -> Option<Self> {
        reduce(queries, Self::Any)
    }

    pub fn with_all(queries: Vec<Query>) -> Option<Self> {
        reduce(queries, Self::All)
    }

    pub fn get_words(&self) -> Vec<String> {
        let mut words = Vec::new();
        self.inner_get_words(&mut words);
        words
    }

    fn inner_get_words(&self, words: &mut Vec<String>) {
        match self {
            Self::Word(word) => words.push(word.clone()),
            Self::Any(queries) | Self::All(queries) => {
                for query in queries {
                    query.inner_get_words(words);
                }
            }
        }
    }
}

pub fn parse_words(line: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut word_start = skip_to_word_start(line);
    while !word_start.is_empty() {
        if let Some((word, rest)) = word_start.split_once(|c: char| !c.is_alphabetic()) {
            if word.len() > 1 {
                words.push(word.to_uppercase());
            }
            word_start = skip_to_word_start(rest);
        } else {
            if word_start.len() > 1 {
                words.push(word_start.to_uppercase());
            }
            break;
        }
    }
    words
}

fn skip_to_word_start(input: &str) -> &str {
    let mut rest = input;
    while !rest.is_empty() {
        rest = if let Some(rest) = strip_ansi_ctrl(rest) {
            rest
        } else if let Some(rest) = strip_long_hex(rest) {
            rest
        } else if let Some(rest) = strip_first_char(rest, |c: char| !c.is_alphabetic()) {
            rest
        } else {
            break;
        };
    }
    rest
}

fn strip_first_char(s: &str, predicate: impl Fn(char) -> bool) -> Option<&str> {
    if let Some(first) = s.chars().next() {
        if predicate(first) {
            return Some(&s[first.len_utf8()..]);
        }
    }
    None
}

fn strip_long_hex(s: &str) -> Option<&str> {
    if let Some((hex, rest)) = s.split_once(|c: char| !c.is_ascii_hexdigit()) {
        if hex.len() > 8 {
            return Some(rest);
        }
    }
    None
}

fn strip_ansi_ctrl(s: &str) -> Option<&str> {
    let rest = if let Some(rest) = s.strip_prefix("\x1b[") {
        rest
    } else if let Some(rest) = s.strip_prefix("\u{1b}[") {
        rest
    } else {
        return None;
    };
    rest.strip_prefix(|c| ('a'..='z').contains(&c) || ('A'..='Z').contains(&c))
}
