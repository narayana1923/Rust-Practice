use rand::Rng;
use std::{cmp::Ordering, io::stdin};

fn main() {
    println!("Guess the number");

    let rand_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input number:");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");

        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(errmsg) => {
                println!("Enter a valid input\nReceived err: {errmsg}");
                break;
            }
        };

        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Aim higher"),
            Ordering::Greater => println!("Don't be greedy"),
            Ordering::Equal => {
                println!("Bravo!!");
                break;
            }
        };
    }
}
