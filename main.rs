use std::io;
use rand::{Rng, rngs::ThreadRng};
use std::cmp::Ordering;


fn main() {
    println!("Guess the number");

    println!("Please input your guess : ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess:i32=guess.trim().parse().expect("Please type a number : ");

    let mut rng: ThreadRng = ThreadRng::default();
    let secret_number: i32 = rng.gen_range(1..=100);

    println!("The secret number is {secret_number}");

    

    println!("You guessed : {guess}");

    match  guess.cmp(&secret_number) {
        Ordering::Less=>println!("Too small"),
        Ordering::Greater=>println!("Too big"),
        Ordering::Equal=>println!("You win!"),
        
    }
}
