use rand::Rng;
use std::io::{self, Write};

fn random_bool() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen::<bool>() // Generates a random boolean value (true or false)
}

fn main() {
    println!("Press Enter To Flip A Coin...");
    let _ = io::stdin().read_line(&mut String::new()).unwrap();

    let outcome = if random_bool() {
        "Heads 🗣"
    } else {
        "Tails 🥠"
    };

    println!("It landed on {}", outcome);
    println!("");
    println!("Press Enter to quit...");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut String::new()).unwrap();
}
