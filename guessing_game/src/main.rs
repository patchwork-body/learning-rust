use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to guessing game!");
    println!("Guess your number:");

    let secret_number = rand::thread_rng().gen_range(1, 100);
    let mut attempts = 10;

    loop {
        if attempts == 0 {
            println!("You lose! Secret number was {}", secret_number);
            break;
        }

        let mut user_guess = String::new();

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read user prompt");

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        attempts -= 1;
        println!("Try again, attempts left: {}", attempts);
    }
}
