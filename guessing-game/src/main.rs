use rand::{thread_rng, Rng};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io;

fn main() {
    let secret: u32 = thread_rng().gen_range(1..=10);
    println!("*** Guess the number ***");
    loop {
        println!("Input your guess:");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {guess}");
        match guess.cmp(&secret) {
            Less => println!("You guessed too low..."),
            Greater => println!("You guessed too high..."),
            Equal => {
                println!("Nice, you won!");
                break;
            }
        }
    }
}
