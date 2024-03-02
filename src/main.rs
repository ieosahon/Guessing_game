use std::cmp::Ordering;
use std::io;
use rand::{random, Rng};

fn main() {
    println!("welcome to guess the number game");

    // generating a random number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");
    let mut count  = 0;
    let mut wrong_guess_count = 0;
    loop {
        println!("Enter a number");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        //let guess :i32 = guess.trim().parse().expect("Please enter a valid number");
        // converting the string number value to an i32 number type, and checking if the input can be converted to i32 data type
        let guess :i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) =>{
                wrong_guess_count += 1;
                if wrong_guess_count >= 3 {
                    println!("Too many wrong inputs. Game over!");
                    break;
                } else {
                    println!("Invalid input. Please enter a number.");
                    continue;
                }
            }
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess number is too small"),
            Ordering::Greater => println!("Guess number is too large"),
            Ordering::Equal => {
                println!("Correct guess");
                break;
            }
        };
        count += 1;
        if count == 4{
            println!("Game over, you lost!");
            break;
        }
    }
}
