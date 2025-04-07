mod file_utils;
mod index;
mod path_utils;
mod print_utils;
mod query;

use crate::index::{IxBuilder, IxReader, ix_path};
use crate::path_utils::resolve_log_files;
use crate::print_utils::print_line;
use crate::query::{Query, parse_words};
use clap::Parser;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "logix",
    about = "Log indexer and searcher",
    long_about = None,
    disable_help_flag = true,
    arg_required_else_help = true
)]
struct Cli {
    /// Path to log file
    #[arg(short, long)]
    path: PathBuf,

    /// Print debug info
    #[arg(long, default_value = "false")]
    debug_print: bool,

    #[arg(short, long, default_value = "0")]
    before: usize,

    #[arg(short, long, default_value = "0")]
    after: usize,

    #[arg(short, long, default_value = "0")]
    context: usize,

    #[arg(short, long, default_value = "0")]
    head: usize,

    #[arg(short, long, default_value = "0")]
    tail: usize,

    /// List of words to search (prefix match)
    #[arg(required = true)]
    words: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    run_on_path(&Cli::parse())?;
    Ok(())
}

fn run_on_path(args: &Cli) -> anyhow::Result<()> {
    let log_paths = resolve_log_files(vec![args.path.clone()])?;
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
        check_index(log_path.clone())?;
        run_on_file(args, log_path)?;
    }
    Ok(())
}

fn run_on_file(args: &Cli, log_path: PathBuf) -> anyhow::Result<()> {
    let Some(query) = Query::parse(&args.words.join(" ")) else {
        return Ok(());
    };
    let ix = IxReader::new(log_path)?;
    if args.debug_print {
        ix.print_debug();
    }
    let mut lines = ix.query(&query)?;
    let before = args.before.max(args.context);
    let after = args.after.max(args.context);
    let words = query.get_words();
    let mut tail_lines = VecDeque::new();
    if args.debug_print {
        lines.print_debug(0);
    }
    let mut show_separator = true;
    let mut processed = 0;
    let head_requested = args.head > 0;
    let tail_requested = args.tail > 0;
    while let Some(line_offset) = lines.next()? {
        if head_requested && processed < args.head || !tail_requested {
            print_line(&ix, line_offset, &words, before, after, &mut show_separator)?;
        }
        if tail_requested && (!head_requested || processed >= args.head) {
            tail_lines.push_back(line_offset);
            if tail_lines.len() > args.tail {
                tail_lines.pop_front();
            }
        }
        processed += 1;
        if !tail_requested && head_requested && processed >= args.head {
            break;
        }
    }
    for line_offset in tail_lines {
        print_line(&ix, line_offset, &words, before, after, &mut show_separator)?;
    }
    Ok(())
}

pub fn check_index(log_path: PathBuf) -> anyhow::Result<()> {
    let ix_path = ix_path(log_path.clone())?;
    if ix_path.exists() {
        return Ok(());
    }

    let log_file = File::open(&log_path)?;
    let log_size = log_file.metadata()?.len();
    let mut log_reader = BufReader::new(log_file);
    let mut ix_builder = IxBuilder::new();
    let mut line_offset = 0u64;
    let mut last_percent = 0;
    loop {
        let percent = line_offset * 100 / log_size;
        if percent != last_percent {
            print!("\rIndexing: {}%\x1b[K", percent);
            std::io::stdout().flush()?;
            last_percent = percent;
        }
        let mut line: String = String::new();
        let len = log_reader.read_line(&mut line)? as u64;
        if len == 0 {
            break;
        };
        for token in parse_words(&line) {
            ix_builder.add_word(token, line_offset);
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
