use rand::prelude::ThreadRng;
use rand::Rng;
use std::io;

fn main() {
    const START: i32 = 0;
    const END: i32 = 100;

    println!("I'm thinking of a number between {START} and {END}. Can you guess it?");
    let mut rng: ThreadRng = rand::thread_rng();

    let random_number: i32 = rng.gen_range(START..END);

    loop {
        let guess: i32 = read_from_input(START, END);
        if guess == random_number {
            println!("You guessed the number!");
            break;
        } else if guess < random_number {
            println!("Too low! Try again.");
        } else {
            println!("Too high! Try again.");
        }
    }
}

fn read_from_input(bottom: i32, top: i32) -> i32 {
    let mut test_value = 0;
    loop {
        let mut input = String::new();
        io::stdin().read_line( & mut input).expect("Failed to read line");
        test_value = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue
            }
        };
        if test_value >= bottom && test_value <= top {
            break test_value;
        }else{
            println!("Please enter a number between {bottom} and {top}");
        }
    }
}
