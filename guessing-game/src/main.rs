/**
 * This is an example from the Rust Book
 */
extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let random: u32 = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Enter your guess:");

        let mut guess = String::with_capacity(100);
        io::stdin()
            .read_line(&mut guess)
            .expect("Input is required");

        // Shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        if guess < 1 || guess > 100 {
            println!("Enter a number between 1 and 100");
            continue;
        }

        match guess.cmp(&random) {
            Ordering::Less => println!("To small"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
            Ordering::Greater => println!("To big"),
        }
    }
}
