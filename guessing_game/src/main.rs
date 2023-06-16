use std::io::stdin;
use rand::Rng;


fn main() {
    println!("guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("please input a guess.");

    let mut guess = String::new();

    stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("You guessed: {guess}");

    println!("The number was {secret_number}!");
}