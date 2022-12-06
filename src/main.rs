use colored::*;
use rand::Rng;
use std::cmp::Ordering; // Enum that will compare
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate a random number
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", secret_number);

    let mut number_of_guesses: u32 = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Take user input using io
        io::stdin() // std gives us a handle to the standart input
            .read_line(&mut guess) // take the user input
            .expect("Failed to read line"); // our error message

        // convert the string input to an integer input
        // trim removes all whitespaces
        // u32 => annotates our value to an integer (unsigned 32-bit integer)
        // when using the same variable (guess) more than once it's called shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        number_of_guesses = number_of_guesses + 1;
        println!("You have guessed: {} {}", number_of_guesses, "times.");
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small! ".red()),
            Ordering::Greater => println!("{}", "Too big! ".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                println!("You guessed: {} {}", number_of_guesses, "times.");
                break;
            }
        }
    }
}
