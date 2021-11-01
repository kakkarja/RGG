use guessing_game::guessing::{Numb, Guess};
use guessing_game::{contag, sect};
use std::{cmp::Ordering, io};

fn run() {

    let mut secret_number = sect();
    let mut trial = 1;
    let mut check = false;
    secret_number.ckeo();
    println!("You have 10 chances to guess!");
    
    loop{
        
        if trial < 11 {
            let mut guess = String::new();
            println!("\nPlease input your guess.");
            
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            
            let guess = Numb::new(
                match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Must be integer!");
                        continue;
                    }
                }
            );

            match guess.num {
                
                num if num == 0 => {
                    println!("Cannot be 0!");
                    continue;
                },
                num if num > 100 => {
                    println!("Cannot exceed 100!");
                    continue;
                },
                _ => {
                    println!("You guessed: {} chance: {}", guess.num, trial);
                    
                    match guess.num.cmp(&secret_number.num) {
                        Ordering::Less => println!("Too small!"),
                        Ordering::Greater => println!("Too big!"),
                        Ordering::Equal => {
                            check = true;
                            println!(
                                "You win! guess {} == secret number {}",
                                guess.num, secret_number.num
                            );
                        }
                    }
                }
            }
        } else {

            check = true;
            println!(
            "You have exceeded the guessing! secret number is {}",
            secret_number.num
            );
        }

        trial += 1;
        
        if check {

            if contag() {
                check = false;
                trial = 1;
                secret_number = sect();
                secret_number.ckeo();
                println!("You have 10 chances to guess!");
                continue;
            } else {
                break;
            }
        }
    }
}

pub fn main() {
    println!("Game start!");
    run();
}