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
}
