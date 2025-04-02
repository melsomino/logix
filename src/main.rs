mod build;
mod file_utils;
mod find;
mod index;
mod path_utils;
mod token;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "logix")]
#[command(about = "Log indexer and searcher", long_about = None)]
struct Cli {
    /// Path to log file
    #[arg(short, long)]
    log: PathBuf,

    /// Print debug info
    #[arg(short, long, default_value = "false")]
    print_debug: bool,

    #[arg(short, long, default_value = "0")]
    before: usize,

    #[arg(short, long, default_value = "0")]
    after: usize,

    #[arg(short, long, default_value = "0")]
    context: usize,

    /// List of words to search (prefix match)
    #[arg(required = true)]
    words: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let before = args.before.max(args.context);
    let after = args.after.max(args.context);
    find::query(
        args.log,
        &args.words.join(" "),
        args.print_debug,
        before,
        after,
    )?;

    Ok(())
}
