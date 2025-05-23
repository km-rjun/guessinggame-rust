use rand::Rng;
use std::io;

fn main() {
    println!("Guess the Number");

    let secret_number:  u32 = rand::rng().random_range(1..=100);
    println!("The Secret Number is: {}", secret_number);

    loop {
        println!("Input Your Guess: ");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to Read Line."); 

        let guess: u32 = guess.trim().parse().expect("Please Type a Valid Number");
        println!("You Guessed: {}", guess);
    }
}
