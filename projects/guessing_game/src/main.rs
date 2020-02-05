use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    // If we hadn’t put the use std::io
    // line at the beginning of the program,
    // we could have written this function call
    // as std::io::stdin. The stdin function
    // returns an instance of std::io::Stdin,
    // which is a type that represents a handle
    // to the standard input for your terminal.
    io::stdin()
    // '&' indicates that this argument is a
    // reference, which gives you a way to let
    // multiple parts of your code access one
    // piece of data without needing to copy
    // that data into memory multiple times.
    //
    // references are immutable by default.
    .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
