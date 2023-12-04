fn main() {
    
    // Expect: grrs <pattern> <path>
    let pattern = std::env::args().nth(1).expect("No pattern given");
    let path = std::env::args().nth(2).expect("No path given");

    println!("Pattern: {:?}\nPath: {:?}", pattern, path);
}