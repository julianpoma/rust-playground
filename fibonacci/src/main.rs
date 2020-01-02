use std::io;

fn main() {
    println!("Enter the amount of iterations");

    let iterations: u32 = loop {
        let mut input = String::with_capacity(100);

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid input as number");
                continue;
            }
        };

        break input;
    };

    println!("{}", fibonacci(iterations));
    println!("{}", fibonacci_loop(iterations));
}

fn fibonacci_loop(count: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;

    for c in { 0..count } {
        let next = a + b;

        if c % 2 == 1 {
            a = next;
        } else {
            b = next;
        }
    }

    if count % 2 == 0 {
        a
    } else {
        b
    }
}

fn fibonacci(number: u32) -> u32 {
    if number == 0 {
        return 0;
    }

    if number == 1 {
        return 1;
    }

    fibonacci(number - 1) + fibonacci(number - 2)
}
