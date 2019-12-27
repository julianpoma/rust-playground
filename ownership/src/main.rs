/**
 * List of data that is stored of the stack
 * The size is known at compile time
 * - Integer
 * - Floats
 * - Hex
 * - Boolean
 * - Array
 * - Tuples
 * - str (can live on the heap, stack or binary, see here https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str/24159933#24159933)
 *
 * Heap data
 * - Vec
 * - String
 * - Box
 *
 * String vs str: https://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html#fn:1
 **/

fn main() {
    ownership();
}

fn ownership() {
    // Here, the value is copied to the variable "y". "x" is still valid
    let x = 23;
    let y = x;
    println!("x = {} and y = {}", x, y);

    // The same happens with other data types
    let x = [1, 2, 3];
    let y = x;
    println!("x = {:?} and y = {:?}", x, y);

    let x = 'x';
    let y = x;
    println!("x = {:?} and y = {:?}", x, y);

    // Like touples, but only if they contain stack or copy type values
    let x = ("hello", 23);
    let y = x;
    println!("x = {:?} and y = {:?}", x, y);

    let x = (String::from("hello"), 23);
    let y = x; // <--- x is not longer valid, since String doesnt implement the copy trait
    println!("y = {:?}", y);

    // What about string literals ? This are also copied
    let x = "hello world";
    let y = x;
    println!("x = {:?} and y = {:?}", x, y);

    // But not on the String type
    let x = String::from("hello string");
    let y = x; // <--- x is not longer valid, we *moved* the value to var "y", not copy/clones
    println!("y = {:?}", y);

    let mut x = String::from("Heap string");

    // We need a scope since we cant have a mutable and inmutable reference at the same time
    {
        let y = &x; // <--- borrowing
        fancy_print(y);
    }

    x.push_str(" , I was mutated!");
    println!("x = {}", x);

    let mut x = String::from("Lovely car");

    {
        let r1 = &x;
        println!("First word: {}", first_word(r1));
    }
    
    let r2 = &mut x;
    add_color(r2);
    println!("Changed: {:?}", x);
}

fn fancy_print(s: &str) -> () {
    println!("fancy print of s = {}", s);
}

fn first_word(my_string: &String) -> &str {
    for (i, &byte) in my_string.as_bytes().iter().enumerate() {
        if byte == b' ' {
            return &my_string[..i];
        }
    }

    &my_string[..]
}

fn add_color(my_string: &mut String) {
    my_string.push_str(" is red");
}
