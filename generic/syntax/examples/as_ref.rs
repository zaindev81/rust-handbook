use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

// Structure to hold search options
struct Args {
    keyword: String,
}

// Function to search inside a file
fn search_in_file<P: AsRef<Path>>(
    args: &Args,
    file_path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    // Convert the generic file_path into a unified &Path reference
    let path = file_path.as_ref();

    // Open the file (if it fails, return immediately using ?)
    let file = File::open(path)?;

    // Use BufReader for efficient line-by-line reading
    let reader = BufReader::new(file);

    // Check each line and print if it contains the keyword
    for (index, line) in reader.lines().enumerate() {
        let line = line?; // Extract Result<String>
        if line.contains(&args.keyword) {
            println!("Line {}: {}", index + 1, line);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Search keyword
    let args = Args {
        keyword: "Rust".to_string(),
        // keyword: String::from("Rust"),
        // keyword: Path::new("Rust"),
    };

    // Search the "sample.txt" file
    search_in_file(&args, "sample.txt")?;

    Ok(())
}


// let opt = Some(String::from("hello"));

// if let Some(v) = opt {
//     println!("{}", v);  // OK
// }
// // opt is now moved and cannot be used again

// let opt = Some(String::from("hello"));

// if let Some(v) = opt.as_ref() {
//     println!("{}", v);  // OK
// }
// // opt is still usable here because we only borrowed it

// let json_str = String::from("{\"name\":\"John\"}");
// let body_to_send = Some(json_str.as_bytes().to_vec());

// println!("{:?}", body_to_send);
// // Output: Some([123, 34, 110, 97, 109, 101, 34, 58, 34, 75, 97, 122, 117, 34, 125])
