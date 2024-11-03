use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("You have 10 turns available to win this game!");

    let mut turn = 10;

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        turn -= 1;
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small! Try Again!");
                println!("{turn} turns left!")
            }

            Ordering::Greater => {
                println!("Too big! Try Again!");
                println!("{turn} turns left")
            }

            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        if turn == 0 {
            println!("You Lose!");
            println!("You have used all your available turns");
            break;
        }
    }
}