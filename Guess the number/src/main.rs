use rand::prelude::*;
use std::io;

fn main() {
    let mut current_guess: i32;
    let current_random_number: i32 = thread_rng().gen_range(1..101);

    let mut not_guessed: bool = true;

    while not_guessed {
        let mut buffer: String = String::new();
        println!("Guess a number between 1 and 100:");
        let _ = io::stdin().read_line(&mut buffer);

        current_guess = buffer.trim().parse::<i32>().unwrap();

        if current_guess == current_random_number {
            println!("Congratulations! Your guess was correct! It was {} and you guessed {}!", current_random_number, current_guess);
            not_guessed = false;
        } else if current_guess > current_random_number {
            println!("Your guess was too high, you guessed {}!", current_guess);
        } else if current_guess < current_random_number {
            println!("Your guess was too low, you guessed {}!", current_guess);
        }
    }
}