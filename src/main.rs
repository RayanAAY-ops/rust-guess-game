use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_number = rand::thread_rng().gen_range(0..10);
    println!("Guess the number!");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // Store user input in guess writing to the ref
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Ignore all errors related to parsing
        };

        println!("you guess {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Bingo");
                break;
            }
        }
    }
}
