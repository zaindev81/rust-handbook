use clap::{Arg, ArgAction, Command, ArgMatches};
use std::fs;
use std::path::{Path, PathBuf};
use std::io;

fn main() {
    let matches = Command::new("file-tool")
        .version("1.0.0")
        .about("A file manipulation tool for copy, move, and delete operations")
        .arg(
            Arg::new("operation")
                .help("Operation to perform")
                .required(true)
                .value_parser(["copy", "move", "delete"])
                .index(1)
        )
        .arg(
            Arg::new("source")
                .help("Source file path")
                .required(true)
                .index(2)
        )
        .arg(
            Arg::new("destination")
                .help("Destination file path (required for copy and move)")
                .index(3)
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .help("Force operation (overwrite existing files)")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(ArgAction::SetTrue)
        )
        .get_matches();

    if let Err(e) = run(&matches) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let operation = matches.get_one::<String>("operation").unwrap();
    let source = matches.get_one::<String>("source").unwrap();
    let destination = matches.get_one::<String>("destination");
    let force = matches.get_flag("force");
    let verbose = matches.get_flag("verbose");

    let source_path = PathBuf::from(source);

    match operation.as_str() {
        "copy" | "move" => {
            if destination.is_none() {
                return Err("Destination path is required for copy and move operations".into());
            }
        }
        "delete" => {
            if destination.is_some() {
                return Err("Destination path is not required for delete operation".into());
            }
        }
        _ => unreachable!(),
    }

    if !source_path.exists() {
        return Err(format!("Source file '{}' does not exist", source).into());
    }

    match operation.as_str() {
        "copy" => {
            let dest_path = PathBuf::from(destination.unwrap());
            copy_file(&source_path, &dest_path, force, verbose)?;
        },
        "move" => {
            let dest_path = PathBuf::from(destination.unwrap());
            move_file(&source_path, &dest_path, force, verbose)?;
        },
        "delete" => {
            delete_file(&source_path, force, verbose)?;
        },
        _ => unreachable!(),
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
            "Destination file already exists"
        ));
    }

    if let Some(parent) = destination.parent() {
        println!("Ensuring parent directory exists: '{}'", parent.display());

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
        println!("Copied '{}' to '{}', size: {} bytes", source.display(), destination.display(), metadata.len());
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
        println!("Ensuring parent directory exists: '{}'", parent.display());

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
        println!("Moved '{}' to '{}', size: {} bytes", source.display(), destination.display(), metadata.len());
    }

    Ok(())
}

fn delete_file(source: &Path, force: bool, verbose: bool) -> io::Result<()> {
    if verbose {
        println!("Deleting file '{}'", source.display());
    }

    if !force {
        print!("Are you sure you want to delete '{}'? (y/N): ", source.display());
        io::Write::flush(&mut self::io::stdout())?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            println!("Deletion cancelled.");
            return Ok(());
        }
    }

    // Check if it's a file or directory
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