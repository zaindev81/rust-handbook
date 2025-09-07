use clap::{Parser, ValueEnum};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// A file manipulation tool for copy, move, and delete operations
#[derive(Parser, Debug)]
#[command(version = "1.0.0", about = "A file manipulation tool for copy, move, and delete operations")]
struct Args {
    /// Operation to perform: copy, move, or delete
    #[arg(value_enum)]
    operation: Operation,

    /// Source file path
    source: String,

    /// Destination file path (required for copy and move)
    destination: Option<String>,

    /// Force operation (overwrite existing files or delete without prompt)
    #[arg(short, long)]
    force: bool,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Clone, ValueEnum)]
enum Operation {
    Copy,
    Move,
    Delete,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(&args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let source_path = PathBuf::from(&args.source);

    match args.operation {
        Operation::Copy | Operation::Move => {
            if args.destination.is_none() {
                return Err("Destination path is required for copy and move operations".into());
            }
        }
        Operation::Delete => {
            if args.destination.is_some() {
                return Err("Destination path is not required for delete operation".into());
            }
        }
    }

    if !source_path.exists() {
        return Err(format!("Source file '{}' does not exist", args.source).into());
    }

    match args.operation {
        Operation::Copy => {
            let dest_path = PathBuf::from(args.destination.as_ref().unwrap());
            copy_file(&source_path, &dest_path, args.force, args.verbose)?;
        }
        Operation::Move => {
            let dest_path = PathBuf::from(args.destination.as_ref().unwrap());
            move_file(&source_path, &dest_path, args.force, args.verbose)?;
        }
        Operation::Delete => {
            delete_file(&source_path, args.force, args.verbose)?;
        }
    }

    Ok(())
}

fn copy_file(source: &Path, destination: &Path, force: bool, verbose: bool) -> io::Result<()> {
    if verbose {
        println!("Copying from '{}' to '{}'", source.display(), destination.display());
    }

    if destination.exists() && !force {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "Destination file already exists",
        ));
    }

    if let Some(parent) = destination.parent() {
        if verbose {
            println!("Ensuring parent directory exists: '{}'", parent.display());
        }

        if !parent.exists() {
            if verbose {
                println!("Creating parent directory '{}'", parent.display());
            }
            fs::create_dir_all(parent)?;
        }
    }

    fs::copy(source, destination)?;

    if verbose {
        let metadata = fs::metadata(source)?;
        println!(
            "Copied '{}' to '{}', size: {} bytes",
            source.display(),
            destination.display(),
            metadata.len()
        );
    }

    Ok(())
}

fn move_file(source: &Path, destination: &Path, force: bool, verbose: bool) -> io::Result<()> {
    if verbose {
        println!("Moving from '{}' to '{}'", source.display(), destination.display());
    }

    if destination.exists() && !force {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Destination file '{}' already exists", destination.display()),
        ));
    }

    if let Some(parent) = destination.parent() {
        if verbose {
            println!("Ensuring parent directory exists: '{}'", parent.display());
        }

        if !parent.exists() {
            if verbose {
                println!("Creating parent directory '{}'", parent.display());
            }
            fs::create_dir_all(parent)?;
        }
    }

    fs::copy(source, destination)?;

    if verbose {
        let metadata = fs::metadata(source)?;
        println!(
            "Moved '{}' to '{}', size: {} bytes",
            source.display(),
            destination.display(),
            metadata.len()
        );
    }

    Ok(())
}

fn delete_file(source: &Path, force: bool, verbose: bool) -> io::Result<()> {
    if verbose {
        println!("Deleting file '{}'", source.display());
    }

    if !force {
        print!("Are you sure you want to delete '{}'? (y/N): ", source.display());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            println!("Deletion cancelled.");
            return Ok(());
        }
    }

    let metadata = fs::metadata(source)?;

    if metadata.is_file() {
        fs::remove_file(source)?;
        if verbose {
            println!("Deleted file '{}'", source.display());
        }
    } else if metadata.is_dir() {
        if force {
            fs::remove_dir_all(source)?;
            if verbose {
                println!("Deleted directory '{}'", source.display());
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Force flag is required to delete directories",
            ));
        }
    }

    Ok(())
}
