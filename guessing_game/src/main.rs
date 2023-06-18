use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("please input a guess.");
        let mut guess = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        
        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Please input a number!");
    
        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

    println!("The number was {secret_number}!");
}