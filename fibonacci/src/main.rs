use std::io;

fn main() {
    println!("Enter the amount of iterations");

    loop {
        let mut iterations = String::with_capacity(100);

        io::stdin()
            .read_line(&mut iterations)
            .expect("Error reading input");

        let iterations: u32 = match iterations.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid input");
                continue;
            }
        };

        for number in 0..iterations + 1 {
            println!("{}", fibonacci(number));
        }

        break;
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
