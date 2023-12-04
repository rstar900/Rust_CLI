use clap::Parser;

#[derive(Parser)]
struct Cli {
    // Pattern string to look for
    pattern: String,
    // Path of the file, in which the pattern is to be searched
    path: std::path::PathBuf
}

fn main() {

    // Expect: grrs <pattern> <path>
    // Instead of using std::env::args().nth(n).expect("message") and parsing manually,
    // we use clap's parsing functionality
    let args = Cli::parse();

    println!("Pattern: {:?}\nPath: {:?}", args.pattern, args.path);
}