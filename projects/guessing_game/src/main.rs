use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn ask_for_guess(guess: &mut String){
    println!("\nPlease input your guess.");
    io::stdin()
        .read_line(guess)
        .expect("\nFailed to read line");
}

fn main() {
    println!("-----------------------");
    println!("Game: Guess the number!");
    println!("-----------------------");

    let secret_number = rand::thread_rng().gen_range(1, 101);    
    
    loop {
        let mut guess = String::new();
        
        ask_for_guess(&mut guess);
        
        println!("You guessed: {}", guess);
        
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number");
        
        // Can't refactor the code below
        // Unable to use keyword `break` outside of a loop
        // `rustc --explain E0268`
        match guess.cmp(secret_number) {
            Ordering::Less => println!("The number {} is too small", guess),
            Ordering::Greater => println!("The number {} is too big", guess),
            Ordering::Equal => {
                println!("The secret num is {}.\nWell done you've guessed correct\n\nYou Win! \n", guess);
                break;
            },
        }
    }
}
