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

    // Reading the file from the path supplied 
    let contents = std::fs::read_to_string(&args.path).expect("Cannot read file from the given path"); 

    // Iterate over the lines and print the line where the pattern is found
    for line in contents.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

}