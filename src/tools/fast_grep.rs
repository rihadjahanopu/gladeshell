// =============================================================================
//  src/tools/fast_grep.rs — Native Parallel Ripgrep Search (`fancybash grep` / `fancybash rg`)
//
//  Powered by ripgrep's `ignore`, `grep-regex`, `grep-searcher`, `grep-printer`.
// =============================================================================

use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

use clap::Args;
use grep_printer::StandardBuilder;
use grep_regex::RegexMatcherBuilder;
use grep_searcher::SearcherBuilder;
use ignore::WalkBuilder;
use termcolor::{BufferWriter, ColorChoice};

#[derive(Args, Debug)]
pub struct GrepArgs {
    /// String or regex pattern to search for
    #[arg(value_name = "PATTERN")]
    pub pattern: String,

    /// Root directory or file path to search
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    /// Include hidden files and directories in search
    #[arg(long)]
    pub hidden: bool,

    /// Do not respect .gitignore or .ignore files
    #[arg(long = "no-ignore")]
    pub no_ignore: bool,

    /// Case-insensitive search
    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,
}

pub fn run(args: GrepArgs) -> Result<(), Box<dyn Error>> {
    // 1. Build regex matcher
    let matcher = RegexMatcherBuilder::new()
        .case_insensitive(args.ignore_case)
        .build(&args.pattern)
        .map_err(|e| format!("Invalid search pattern regex '{}': {e}", args.pattern))?;

    // 2. Configure parallel walker
    let threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    let mut walk_builder = WalkBuilder::new(&args.path);
    walk_builder
        .hidden(!args.hidden)
        .git_ignore(!args.no_ignore)
        .ignore(!args.no_ignore)
        .threads(threads);

    let walker = walk_builder.build_parallel();

    // 3. Thread-safe printer output writer
    let writer = Arc::new(BufferWriter::stdout(ColorChoice::Auto));
    let matcher = Arc::new(matcher);

    // 4. Execute parallel worker search loops
    walker.run(|| {
        let matcher = Arc::clone(&matcher);
        let writer = Arc::clone(&writer);
        let mut searcher = SearcherBuilder::new().build();

        Box::new(move |result| {
            let entry = match result {
                Ok(entry) => entry,
                Err(err) => {
                    eprintln!("Traversal error: {err}");
                    return ignore::WalkState::Continue;
                }
            };

            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                let path = entry.path();
                let mut buffer = writer.buffer();
                let mut printer = StandardBuilder::new().build(&mut buffer);

                if let Err(err) = searcher.search_path(&*matcher, path, printer.sink(&*matcher)) {
                    eprintln!("Error reading file {}: {err}", path.display());
                } else if !buffer.is_empty() {
                    let _ = writer.print(&buffer);
                }
            }

            ignore::WalkState::Continue
        })
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_fast_grep_valid_pattern() {
        let temp_dir = std::env::temp_dir();
        let test_file_path = temp_dir.join("fancybash_fast_grep_test.txt");
        {
            let mut file = File::create(&test_file_path).unwrap();
            writeln!(file, "hello fancybash fast grep").unwrap();
        }

        let args = GrepArgs {
            pattern: "fancybash".to_string(),
            path: test_file_path.clone(),
            hidden: true,
            no_ignore: true,
            ignore_case: false,
        };

        assert!(run(args).is_ok());
        let _ = std::fs::remove_file(test_file_path);
    }

    #[test]
    fn test_fast_grep_invalid_regex() {
        let args = GrepArgs {
            pattern: "[invalid regex".to_string(),
            path: PathBuf::from("."),
            hidden: false,
            no_ignore: false,
            ignore_case: false,
        };

        assert!(run(args).is_err());
    }
}

