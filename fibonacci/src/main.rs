use std::io;

fn main() {
    println!("Enter the amount of iterations");

    let mut iterations = String::with_capacity(100);

    io::stdin().read_line(&mut iterations).expect("Please enter a valid input");

    let iterations: u32 = match iterations.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Please enter a valid input")
        },
    };

    for number in 0..iterations + 1 {
        let x = fibonacci(number);
        println!("{}", x);
    }
}

fn fibonacci (number: u32) -> u32 {
    if number == 0 {
        return 0;
    }

    if number == 1 {
        return 1;
    }

    fibonacci(number - 1) + fibonacci(number - 2)
}
