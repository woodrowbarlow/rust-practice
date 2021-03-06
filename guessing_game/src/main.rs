extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn read_i32() -> i32 {
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(n) => return n,
            Err(_) => println!("Please input a number."),
        };
    }
}

fn read_guess(min: i32, max: i32) -> i32 {
    loop {
        let guess = read_i32();
        if guess < min {
            println!("Please input a number that is at least {}.", min);
        } else if guess > max {
            println!("Please input a number that is no larger than {}.", max);
        } else {
            return guess;
        }
    }
}

fn main() {
    println!("Guess the number!");
    let (min, max) = (1, 10);
    let secret_number = rand::thread_rng().gen_range(min, max + 1);
    loop {
        println!("Please input your guess, between {} and {}.", min, max);
        let guess = read_guess(min, max);
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            },
        };
    }
}
