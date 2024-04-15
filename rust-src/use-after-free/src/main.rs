use std::env;

fn consume(s: String) {
    println!("String: ('{}', {})", s,s.len());
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        println!("Usage: cargo run <string>");
        return;
    }
    let s: String = argv[1].to_string();
    consume(s);
    consume(s);
}
