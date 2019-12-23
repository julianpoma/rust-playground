/**
 * This is an example from the Rust Book
 */
extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Welcome! Enter your guess:");

    let mut guess = String::with_capacity(100);

    let random: u32 = rand::thread_rng().gen_range(1, 101);

    io::stdin()
        .read_line(&mut guess)
        .expect("Input is required");

    let trim_guess = guess.trim();

    let int_guess: u32 = trim_guess.parse().unwrap();

    if int_guess == random {
        println!("Your guess {} was correct!", guess);
    } else {
        println!("Your guess {} was incorrect: {}", guess, random);
    }
}
