use std::env;
fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        println!("Usage: cargo run <guess>");
        return;
    }
    let secret: String = String::from("letmein");
    let guess: String = argv[1].clone();
    if secret == guess{
        println!("You guessed correctly!");
        println!("The secret was: {}" ,secret);
    }else{
        println!("'{}' was not the secret.",guess);
        println!("Try again later!\n");
    }
}
