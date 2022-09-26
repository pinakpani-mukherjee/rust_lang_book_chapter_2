use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess The Number!!!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input a guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line !!!!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small :(".red()),
            Ordering::Equal => {
                println!("{}", "You win !!!".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big :(".red()),
        }
    }
}
