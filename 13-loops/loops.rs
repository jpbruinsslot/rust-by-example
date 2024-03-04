// Title: Loops
// Language: rust
//
// Rust has three kinds of loops: loop, while, and for.

fn main() {

    // The `loop` keyword tells Rust to execute a block of code over and over
    // forever, or until you explicitly tell it to stop.
    loop {
        println!("hello, world!");
        break;
    }

    // You can add the value you want returned after the break expression you
    // use to stop the loop; that value will be returned out of the loop so you
    // can use it.
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // A while loop runs as long as a condition holds true.
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    // A `for` loop is used to iterate through a collection of items.
    let array = [1, 2, 3, 4, 5];
    for element in array {
        println!("{} ", element);
    }

    // You can also use a `for` loop to iterate over the elements of a range,
    // the `..` is exclusive.
    for i in 0..5 {
        println!("{} ", i);
    }

    // And n use `..=` is inclusive and will include the last number.
    for i in 0..=5 {
        println!("{}", i);
    }
}
