use std::cmp::Ordering;
use std::io;
// prelude includes the basic functionality of rand that will be most used idk about ::*
use rand::prelude::*;

fn main() {
    println!(" i hate this game.");
    // rand::rng is pretty self explanatory along with .random_range
    let secret_number: u32 = rand::rng().random_range(1..=100);
    loop {
        println!(" Choose a random number from 1-100 ");
        let mut word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to catch line.");
        // to make it so that the io accepts integers too
        let guess: u32 = match word.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("not an int or invalid value");
                return;
            }
        };

        println!("Your number is {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("cold"),
            Ordering::Greater => println!("hot"),
            Ordering::Equal => {
                println!("you won!");
                break;
            }
        }
    }
}
