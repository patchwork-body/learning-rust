use std::io;

fn main() {
    println!("Welcome to guessing game!");
    println!("Guess your number:");

    let mut user_guess = String::new();

    io::stdin()
        .read_line(&mut user_guess)
        .expect("Failed to read user prompt");

    println!("Your guess is {}", user_guess);
}
