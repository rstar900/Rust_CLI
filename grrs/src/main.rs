struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

fn main() {

    // Expect: grrs <pattern> <path>
    let pattern = std::env::args().nth(1).expect("No pattern given");
    let path = std::env::args().nth(2).expect("No path given");

    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path)
    };

    println!("Pattern: {:?}\nPath: {:?}", args.pattern, args.path);
}