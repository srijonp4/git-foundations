#![allow(unused)]
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("number : {secret_number}");
    loop {
        let mut guess = String::new();
        println!("enter your guess : ");
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to readline");
        // let guess: i32 = guess.trim().parse().expect("failed");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your guess is : {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Greater => {
                println!("too big");
            }
            Ordering::Equal => {
                println!("you win");
                break;
            }
            Ordering::Less => {
                println!("too small , try again")
            }
        }
    }
}
