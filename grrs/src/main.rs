use clap::Parser; // Required for parsing command line args
use anyhow::{Context, Result}; // Required for error with context

// For BufReader
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Parser)]
struct Cli {
    // Pattern string to look for
    pattern: String,
    // Path of the file, in which the pattern is to be searched
    path: std::path::PathBuf
}

fn main() -> Result<()> {

    // Expect: grrs <pattern> <path>
    // Instead of using std::env::args().nth(n).expect("message") and parsing manually,
    // we use clap's parsing functionality
    let args = Cli::parse();

    // Reading the file from the path supplied 
    let file = File::open(&args.path).with_context(|| format!("Could not read file `{}`", args.path.display()))?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut bytes_read = reader.read_line(&mut line).with_context(|| format!("Could not read line"))?;

    // Iterate over the lines and print the line where the pattern is found
    while bytes_read != 0 {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
        line.clear(); // important to clear as read_line appends to existing buffer
        bytes_read= reader.read_line(&mut line).with_context(|| format!("Could not read line"))?;
    }

    Ok(())
}