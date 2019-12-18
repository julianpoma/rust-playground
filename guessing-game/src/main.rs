/**
 * This is an example from the Rust Book
 */
use std::io;

fn main() {
    println!("Welcome! Enter your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Input is required");

    println!("{:?}", guess)
}
