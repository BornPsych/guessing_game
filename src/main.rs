use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing the Number!\n\n\n");

    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("\n\nPlease Input the Number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Yeah! You guessed the correct answer");
                break;
            }
            Ordering::Less => {
                println!("Low! The number you guessed is too low, guess the higher number")
            }
            Ordering::Greater => {
                println!("High! The number you guessed is too high, guess the lower number")
            }
        }
    }
}
