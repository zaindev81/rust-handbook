use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    /// Show lines that don't match the pattern(reverse)
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
        if let Err(e) = search_in_file(&args, &file_path) {
            eprintln!("Error searching in file '{}': {}", file_path, e);
        }
    }
}

fn search_in_file(args: &Args, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    // Wraps it in BufReader for efficient line-by-line reading.
    let reader = BufReader::new(file);

    // "Case sensitive" means that uppercase and lowercase letters are treated as different characters.
    let pattern = if args.case_sensitive {
        args.pattern.clone()
    } else {
        args.pattern.to_lowercase()
    };

    let mut matches = Vec::new();
    let mut total_matches = 0;

    for (line_number, line) in reader.lines().enumerate() {
        // println!("Processing line {} in file '{}'", line_number + 1, file_path);
        // println!("Line content: {:?}", line);

        let line = line?;
        let search_line = if args.case_sensitive {
            line.clone()
        } else {
            line.to_lowercase()
        };

        let is_match = search_line.contains(&pattern);
        // let should_include = if args.invert { is_match } else { !is_match };
        let should_include = if args.invert {
            is_match
        } else {
            !is_match
        };

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
            println!("{}: {} matches", file_path, total_matches);
        } else {
            println!("{} matches", total_matches);
        }
    } else {
        for (line_num, line) in matches {
            let mut output = String::new();

            if multiple_files {
                output.push_str(&format!("{}: ", file_path));
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
            files: vec![file.path().to_string_lossy().to_string()],
            case_sensitive: true,
            line_numbers: false,
            count: false,
            invert: false,
        };

        assert!(search_in_file(&args, &file.path().to_string_lossy()).is_ok());
    }

    #[test]
    fn test_case_sensitive() {
        let file = create_test_file("Error\nerror\nERROR");
        let args = Args {
            pattern: "error".to_string(),
            files: vec![file.path().to_string_lossy().to_string()],
            case_sensitive: true,
            line_numbers: false,
            count: false,
            invert: false,
        };

        assert!(search_in_file(&args, &file.path().to_string_lossy()).is_ok());
    }
}