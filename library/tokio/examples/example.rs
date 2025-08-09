#![warn(rust_2018_idioms)]

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}