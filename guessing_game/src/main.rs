use std::cmp::Ordering;
use std::io::{ self, Write };
use rand::Rng;

fn main() {
    loop {
        let secret_number: u8 = rand::thread_rng().gen_range(1..=100);
        println!("Guess the number!");
        print!("Please input your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Large!"),
            Ordering::Equal => {
                println!("You guessed correctly!");
                break;
            }
        }
    }
}
