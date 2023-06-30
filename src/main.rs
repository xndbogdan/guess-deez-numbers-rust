extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    let mut tries = 10;
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    while tries > 0 {
        if guess.trim().parse::<u32>().unwrap() == secret_number {
            println!("You guessed correctly!");
            break;
        } else {
            if guess.trim().parse::<u32>().unwrap() > secret_number {
                println!("You guessed incorrectly!\nYour guess was too high.\nYou have {} tries left.", tries);
            } else {
                println!("You guessed incorrectly!\nYour guess was too low.\nYou have {} tries left.", tries);
            }
            println!("Please input your guess.");

            guess = String::new();

            io::stdin().read_line(&mut guess)
                .expect("Failed to read line");
            tries -= 1;
        }
    }

    if tries == 0 {
        println!("You ran out of tries!\nThe correct number was {}.", secret_number);
    }
}