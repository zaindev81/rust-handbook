use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "grep-lite")]
#[command(about = "A lightweight text search tool")]
#[command(version = "1.0")]
struct Args {
    /// Search pattern
    pattern: String,

    /// File paths to search in
    files: Vec<String>,

    /// Case sensitive search
    #[arg(long)]
    case_sensitive: bool,

    /// Show line numbers
    #[arg(long)]
    line_numbers: bool,

    /// Show only count of matching lines
    #[arg(long)]
    count: bool,

    /// Show lines that don't match the pattern (reverse)
    #[arg(long)]
    invert: bool,
}

fn main() {
    let args = Args::parse();

    if args.files.is_empty() {
        eprintln!("Error: No files provided to search in.");
        std::process::exit(1);
    }

    for file_path in &args.files {
        if let Err(e) = search_in_file(&args, file_path) {
            eprintln!("Error searching in file '{}': {}", file_path, e);
        }
    }
}

/// Search lines in a file according to options.
/// Accepts anything that can be referenced as a Path.
fn search_in_file<P: AsRef<Path>>(
    args: &Args,
    file_path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = file_path.as_ref();
    let file = File::open(path)?;
    // Wrap in BufReader for efficient line-by-line reading.
    let reader = BufReader::new(file);

    // "Case sensitive" means uppercase and lowercase are treated differently.
    let pattern = if args.case_sensitive {
        args.pattern.clone()
    } else {
        args.pattern.to_lowercase()
    };

    let mut matches = Vec::new();
    let mut total_matches = 0usize;

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        let search_line = if args.case_sensitive {
            line.clone()
        } else {
            line.to_lowercase()
        };

        let is_match = search_line.contains(&pattern);

        let should_include = if args.invert { !is_match } else { is_match };

        if should_include {
            total_matches += 1;
            if !args.count {
                // save the line and its number if not counting
                matches.push((line_number + 1, line));
            }
        }
    }

    // Output results
    let multiple_files = args.files.len() > 1;

    if args.count {
        if multiple_files {
            println!("{}: {} matches", path.display(), total_matches);
        } else {
            println!("{}", total_matches);
        }
    } else {
        for (line_num, line) in matches {
            let mut output = String::new();

            if multiple_files {
                output.push_str(&format!("{}: ", path.display()));
            }

            if args.line_numbers {
                output.push_str(&format!("{}:", line_num));
            }

            output.push_str(&line);
            println!("{}", output);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    #[test]
    fn test_basic_search() {
        let file = create_test_file("Hello world\nThis is an error\nAnother line");
        let args = Args {
            pattern: "error".to_string(),
            files: vec![file.path().display().to_string()],
            case_sensitive: true,
            line_numbers: false,
            count: false,
            invert: false,
        };

        assert!(search_in_file(&args, file.path()).is_ok());
    }

    #[test]
    fn test_case_sensitive() {
        let file = create_test_file("Error\nerror\nERROR");
        let args = Args {
            pattern: "error".to_string(),
            files: vec![file.path().display().to_string()],
            case_sensitive: true,
            line_numbers: false,
            count: false,
            invert: false,
        };

        assert!(search_in_file(&args, file.path()).is_ok());
    }

    #[test]
    fn test_invert_logic() {
        let file = create_test_file("foo\nbar\nfoo bar");
        let args = Args {
            pattern: "foo".to_string(),
            files: vec![file.path().display().to_string()],
            case_sensitive: true,
            line_numbers: false,
            count: true,
            invert: true,
        };
        assert!(search_in_file(&args, file.path()).is_ok());
    }
}
