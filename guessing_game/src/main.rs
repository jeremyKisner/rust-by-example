use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let total_allowed_guesses = 10;
    let mut guess_attempts = 0;

    loop {
        println!("please input a guess.");
        let mut guess = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
    
        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }

        guess_attempts += 1;
        
        match guess_attempts.cmp(&total_allowed_guesses) {
            Ordering::Less => println!("Try again ({guess_attempts}/{total_allowed_guesses})!"),
            Ordering::Greater => {
                println!("You exceeded max guesses. You lose!");
                break;
            },
            Ordering::Equal => {
                println!("You exceeded max guesses. You lose!");
                break;
            },
        }
    }

    println!("The number was {secret_number}!");
}