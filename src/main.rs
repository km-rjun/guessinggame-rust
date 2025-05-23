use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number");

    let secret_number:  u32 = rand::rng().random_range(1..=100);
    // println!("The Secret Number is: {}", secret_number);

    loop {
        println!("Input Your Guess: ");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to Read Line."); 

        let guess: u32 = guess.trim().parse().expect("Please Enter a Valid Number");
        println!("You Guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small !!"),
            Ordering::Greater => println!("Too Big !!"),
            Ordering::Equal => {
                println!("You Guessed Correctly !");
                break;
            }
        }
    }
}
